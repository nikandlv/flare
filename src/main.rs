extern crate postgres;

mod service;
mod router;
mod flare;

fn main() -> io::Result<()> {
    flare.spark();
}
