use actix_web::{middleware, App, HttpServer};
use r2d2_postgres::{PostgresConnectionManager,TlsMode};
use r2d2_mysql::{MysqlConnectionManager};
use std::io;
use mysql::{OptsBuilder, Opts};
use std::collections::HashMap;
use std::sync::Mutex;

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



pub fn spark() -> io::Result<()> {
    let sys = actix_rt::System::new("flare");
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let pool= if url.contains("postgresql") {
         build_postgres_pool(&url);
    } else if url.contains("mysql") {
        build_mysql_pool(&url);
    };


    let address = std::env::var("ADDRESS").expect("ADDRESS not defined");
    HttpServer::new(move || {
        let mut state = crate::state::State {
            database: pool.clone(),
            handlers: Mutex::new(HashMap::new()),
            storage: Mutex::new(HashMap::new()),
        };

        App::new()
            .configure(    |cfg| {
                crate::modules::boot_modules(&mut state,cfg);
            })
            .data(state)
            .wrap(middleware::Logger::default())
    })
    .bind(address)?
    .start();

    sys.run()
}