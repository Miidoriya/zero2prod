use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
