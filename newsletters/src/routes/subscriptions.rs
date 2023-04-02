use actix_web::{Responder, HttpResponse, web};
use sqlx::{PgPool};
use tracing::Instrument;
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
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "adding new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`    
    let _request_span_guard = request_span.enter();

    tracing::info!("saving new subscriber details in the database");

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "saving new subscriber details in the database"
    );

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
        .instrument(query_span)
        .await {
            Ok(_) => {
                tracing::info!("new subscriber details have been saved");
                HttpResponse::Ok().finish()
            },
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }

    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span
    
}
