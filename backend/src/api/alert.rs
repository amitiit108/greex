use actix_web::{post, get, delete, web, HttpResponse, Responder};
use crate::db;
use crate::models::alert::Alert;
use crate::AppState;

#[post("/alert")]
async fn create_alert(data: web::Data<AppState>, alert: web::Json<Alert>) -> impl Responder {
    let db_pool = data.db_pool.lock().unwrap();
    match db::alert::insert_alert(&db_pool, &alert.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/alerts")]
async fn get_alerts(data: web::Data<AppState>) -> impl Responder {
    let db_pool = data.db_pool.lock().unwrap();
    match db::alert::fetch_alerts(&db_pool).await {
        Ok(alerts) => HttpResponse::Ok().json(alerts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/alert/{id}")]
async fn delete_alert(data: web::Data<AppState>, alert_id: web::Path<i32>) -> impl Responder {
    let db_pool = data.db_pool.lock().unwrap();
    match db::alert::delete_alert(&db_pool, *alert_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
