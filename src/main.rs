#[macro_use]
extern crate diesel;

#[path = "./service/livreAPI.rs"] mod livre;
#[path = "./service/startAPI.rs"] mod bienvenue;
#[path = "./service/handlers.rs"] mod handlers;
#[path = "./service/models.rs"] mod models;
#[path = "./service/schema.rs"] mod schema;
#[path = "./service/errors.rs"] mod errors;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

    /*HttpServer::new(|| {
        App::new()
            .service(bienvenue::start)
            .service(livre::post)
            .service(livre::delete)
            .service(livre::edit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await*/
}
