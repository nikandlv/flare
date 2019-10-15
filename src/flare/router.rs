use actix_web::{web};

pub fn get(cfg: &mut web::ServiceConfig) {
    crate::modules::boot_modules(cfg);
    cfg.service(
        web::scope("/")
    );
}