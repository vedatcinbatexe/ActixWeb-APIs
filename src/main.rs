mod models;
mod services;
mod error;

// Change your imports to this:
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_web::error::InternalError; // Import this specifically to avoid collision
use std::sync::Mutex;
use std::fs;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let initial_tasks: Vec<models::Task> = if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let app_data = web::Data::new(models::AppState {
        tasks: Mutex::new(initial_tasks),
    });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json("Invalid JSON format!")
                ).into()
            }))
            .service(services::list_tasks)
            .service(services::add_task)
            .service(services::delete_task)
            .service(services::update_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}