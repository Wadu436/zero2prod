use actix_web::{
    web::{self, Form},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>, _connection: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions(id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
        uuid::Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            println!("Failed to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
