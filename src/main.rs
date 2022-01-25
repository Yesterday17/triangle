mod config;

use std::path::PathBuf;
use std::str::FromStr;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use clap::Parser;
use uuid::Uuid;
use crate::config::{AppState, Config};
use askama_actix::TemplateIntoResponse;

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Found().header("location", format!("/{}", data.first())).finish()
}

#[get("/{uuid}")]
async fn quiz(web::Path(uuid): web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match Uuid::from_str(&uuid) {
        Ok(uuid) => {
            data.get(&uuid).unwrap().into_response()
        }
        Err(_) => Ok(HttpResponse::NotFound().finish())
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    config: PathBuf,

    /// Path to the config uuid lock file
    #[clap(short, long)]
    lock: Option<PathBuf>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let lock_path = match args.lock {
        Some(lock_path) => lock_path,
        None => args.config.with_extension("lock"),
    };

    let data = web::Data::new(Config::new(&args.config, &lock_path).into_state());
    HttpServer::new(move || App::new()
        .app_data(data.clone())
        .service(index)
        .service(quiz))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}