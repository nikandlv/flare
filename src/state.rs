use futures::Future;
use std::collections::HashMap;
use std::any::Any;
use std::sync::Mutex;

pub struct State<'a> {
    pub database: (),
    pub handlers: Mutex<HashMap<String,&'a dyn Fn(serde_json::Value)>>,
    pub storage: Mutex<HashMap<String,serde_json::Value>>
}