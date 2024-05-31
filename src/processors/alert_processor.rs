use crate::db;
use crate::models::alert::Alert;
use crate::binance::client::BinanceClient;
use crate::notifications::notifier::Notifier;

pub struct AlertProcessor {
    client: BinanceClient,
    notifier: Notifier,
}

impl AlertProcessor {
    pub fn new(client: BinanceClient, notifier: Notifier) -> Self {
        AlertProcessor { client, notifier }
    }

    pub async fn process_alerts(&self, db_pool: &db::DbPool) {
        let alerts = db::alert::fetch_alerts(db_pool).await.unwrap();

        for alert in alerts {
            self.check_alert(alert, db_pool).await;
        }
    }

    async fn check_alert(&self, alert: Alert, db_pool: &db::DbPool) {
        let current_value = match self.client.get_value(&alert.symbol, &alert.basis, alert.ma_length).await {
            Ok(value) => value,
            Err(_) => return,
        };

        let should_notify = match alert.direction.as_str() {
            "UP" => current_value > alert.value,
            "DOWN" => current_value < alert.value,
            _ => false,
        };

        if should_notify {
            self.notifier.send_notification(&format!(
                "Alert triggered for {}: current value is {}",
                alert.symbol, current_value
            ));

            db::alert::update_alert_status(db_pool, alert.id.unwrap(), "completed").await.unwrap();
        }
    }
}
