  
use actix_web::{web};

pub fn get(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
    );
}