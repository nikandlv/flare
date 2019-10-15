use super::Module;
use actix_web::web;

pub struct Core;

impl Module for Core {
    fn boot(&self,cfg: &mut web::ServiceConfig) -> String {
        String::from("Okay!")
    }
    fn routes(&self,cfg: &mut web::ServiceConfig) {

    }
}