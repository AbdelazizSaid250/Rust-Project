use std::env;

use actix_web::{App, HttpServer, middleware::Logger, web::Data, web::JsonConfig};
use paperclip::actix::OpenApiExt;

use exam::config::start_tracing;
use exam::handler::routes;
use yugabyte::db_connection::CoreDBPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_tracing();
    let core_db_pool_data = Data::new(CoreDBPool::default());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4096))
            .app_data(core_db_pool_data.clone())
            .wrap_api()
            .configure(routes)
            .with_json_spec_at(env::var("OPEN_API").unwrap().as_str())
            .build()
    })
        .bind(format!("{}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap()))
        .expect("Server binding exception")
        .run()
        .await
}
