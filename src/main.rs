extern crate postgres;

mod service;
mod router;
mod flare;
/// Async request handler. Ddb pool is stored in application state.


fn main() -> io::Result<()> {
    flare.spark();
}
