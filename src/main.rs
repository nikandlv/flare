extern crate postgres;

use std::io;
use dotenv;

mod flare;
mod router;
fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    env_logger::init();
    flare::spark()
}
