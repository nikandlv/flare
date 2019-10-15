use actix_web::web;
use crate::state::State;

pub mod core;
pub mod authentication;

pub trait Module {
    fn register(&self,state : &mut State,cfg: &mut web::ServiceConfig);
    fn boot(&self,state : &mut State,cfg: &mut web::ServiceConfig);
    fn routes(&self,state :&mut State,cfg: &mut web::ServiceConfig);
}


pub fn boot_modules(state : &mut  State,cfg: &mut web::ServiceConfig) {
    let modules : Vec<& dyn Module> = vec![&core::Core,&authentication::Authentication];
    for module in modules.iter() {
        module.register(state,cfg);
    }
    for module in modules.iter() {
        module.boot(state,cfg);
        module.routes(state,cfg);
    }
}