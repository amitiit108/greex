use actix_web::web;

mod alert;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(alert::create_alert)
            .service(alert::get_alerts)
            .service(alert::delete_alert),
    );
}
