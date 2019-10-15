use super::Module;
use actix_web::{web, HttpResponse, Error};
use crate::state::State;

pub struct Core;

use serde::{Deserialize, Serialize};
use futures::{Future, future::ok};

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
    fn register(&self,state : &State,cfg: &mut web::ServiceConfig) {

    }
    fn boot(&self,state : &State,cfg: &mut web::ServiceConfig) {

    }
    fn routes(&self,state : &State,cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("core").route("/", web::get().to_async(index)));
    }
}