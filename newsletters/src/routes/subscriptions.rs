use actix_web::{Responder, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email : String,
    name  : String
}

/*
    handler to process user subscribe to newsletters
 */
pub async fn subscriptions(form : actix_web::web::Form<FormData>) -> impl Responder{
    HttpResponse::Ok().finish()
}
