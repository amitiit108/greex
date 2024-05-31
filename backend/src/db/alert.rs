use diesel::prelude::*;
use diesel::result::Error;
use crate::models::alert::Alert;
use crate::db::schema::alerts::dsl::*;
use crate::DbPool;

pub async fn insert_alert(pool: &DbPool, new_alert: &Alert) -> Result<(), Error> {
    let conn = pool.get().unwrap();
    diesel::insert_into(alerts)
        .values(new_alert)
        .execute(&conn)?;
    Ok(())
}

pub async fn fetch_alerts(pool: &DbPool) -> Result<Vec<Alert>, Error> {
    let conn = pool.get().unwrap();
    let results = alerts.load::<Alert>(&conn)?;
    Ok(results)
}

pub async fn delete_alert(pool: &DbPool, alert_id: i32) -> Result<(), Error> {
    let conn = pool.get().unwrap();
    diesel::delete(alerts.find(alert_id)).execute(&conn)?;
    Ok(())
}

pub async fn update_alert_status(pool: &DbPool, alert_id: i32, new_status: &str) -> Result<(), Error> {
    let conn = pool.get().unwrap();
    diesel::update(alerts.find(alert_id))
        .set(status.eq(new_status))
        .execute(&conn)?;
    Ok(())
}
