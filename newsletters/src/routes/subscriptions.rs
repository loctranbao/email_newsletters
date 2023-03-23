use actix_web::{Responder, HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email : String,
    name  : String
}

/*
    handler to process user subscribe to newsletters
 */
pub async fn subscriptions(form : actix_web::web::Form<FormData>,
    pool: web::Data<PgPool>) -> impl Responder{
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
        )
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(pool.get_ref())
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    
}
