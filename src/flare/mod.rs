use actix_web::{middleware, App, HttpServer};
use r2d2_postgres::{PostgresConnectionManager,TlsMode};
use r2d2_mysql::{MysqlConnectionManager};
use std::io;
use mysql::{OptsBuilder, Opts};


pub fn spark() -> io::Result<()> {
    fireup()
}

fn build_postgres_pool(url : &str) -> r2d2::Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(url,TlsMode::None)
        .expect("Unable to connect to database");
    r2d2::Pool::new(manager).expect("Unable to connect to database")
}

fn build_mysql_pool(url : &str) -> r2d2::Pool<MysqlConnectionManager> {
    let options = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(options);
    let manager = MysqlConnectionManager::new(builder);
    r2d2::Pool::new(manager).expect("Unable to connect to database")
}

fn fireup() -> io::Result<()> {
    let sys = actix_rt::System::new("flare");
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let pool= if url.contains("postgresql") {
         build_postgres_pool(&url);
    } else if url.contains("mysql") {
        build_mysql_pool(&url);
    };

    let address = std::env::var("ADDRESS").expect("ADDRESS not defined");
    HttpServer::new(move || {
        App::new()
            .configure(crate::router::get)
            .data(pool.clone())
            .wrap(middleware::Logger::default())
    })
    .bind(address)?
    .start();

    sys.run()
}