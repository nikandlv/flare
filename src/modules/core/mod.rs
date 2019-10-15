use super::Module;
use actix_web::{web, HttpResponse, Error};
use crate::state::State;

pub struct Core;

use serde::{Deserialize, Serialize};
use futures::{Future, future::ok};
use std::any::Any;
use std::sync::{Arc, Mutex};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

fn index() -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok().json(MyObj{
        name: "Hell yea".to_owned()
    })
    )

}



impl Module for Core {
    fn register<'a>(&self, state: &mut State, cfg: &mut web::ServiceConfig) {
        fn register_boot(data: serde_json::Value) {

        }
        state.handlers.lock().unwrap().insert( String::from("boot"), &register_boot);
    }
    fn boot(&self,state : &mut State,cfg: &mut web::ServiceConfig) {
        let callback = state.handlers.get_mut().ok().unwrap().get("boot").unwrap();
        callback(String::from("{\"test\":1}").parse().unwrap());
    }
    fn routes(&self,state : &mut State,cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("core").route("/", web::get().to_async(index)));
    }
}