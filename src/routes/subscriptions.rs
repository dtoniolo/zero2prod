use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>, db_pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as new subscriber.",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database", request_id
    );
    let response = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;

    match response {
        Ok(_) => {
            log::info!(
                "request_id {} - New subscriber details have been saved", request_id
            );
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
