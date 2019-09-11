use uuid;
use futures::Future;
use actix_web::{web, Error, Responder};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

pub fn handle(
    path: web::Path<String>,
    db: web::Data<Pool<PostgresConnectionManager>>,
) -> impl Responder {
    // execute sync code in threadpool
    String::from("test")
}
