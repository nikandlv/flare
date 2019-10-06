use actix_web::{middleware, App, HttpServer};
use r2d2_postgres::{PostgresConnectionManager,TlsMode};
use std::io;


pub fn spark() -> io::Result<()> {
    fireup()
}

fn build_pool() -> r2d2::Pool<PostgresConnectionManager> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let manager = PostgresConnectionManager::new(url,TlsMode::None)
        .expect("Unable to connect to database");
    r2d2::Pool::new(manager).expect("Unable to connect to database")
}

fn fireup() -> io::Result<()> {
    let sys = actix_rt::System::new("flare");
    let pool = build_pool();
    let address = std::env::var("ADDRESS").expect("ADDRESS not defined");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(crate::router::get)
            .wrap(middleware::Logger::default())
    })
    .bind(address)?
    .start();

    sys.run()
}