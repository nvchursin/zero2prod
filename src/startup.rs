use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, services, web};
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(services![health_check, subscribe])
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
