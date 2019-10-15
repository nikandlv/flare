use actix_web::web;

pub mod core;

const MODULES: &'static [&'static dyn Module] = &[&core::Core];

pub trait Module {
    fn boot(&self,cfg: &mut web::ServiceConfig) -> String {
        String::from("Stub")
    }
    fn routes(&self,cfg: &mut web::ServiceConfig) {}
}


pub fn boot_modules(cfg: &mut web::ServiceConfig) {
    for module in MODULES.iter() {
        println!("{}", module.boot(cfg));
    }
}