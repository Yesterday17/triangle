mod config;
mod watch;

use crate::config::{AppState, Config};
use crate::watch::watch;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama_actix::TemplateIntoResponse;
use clap::Parser;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

#[get("/")]
async fn index(data: web::Data<RwLock<AppState>>) -> impl Responder {
    HttpResponse::Found()
        .header("location", format!("/{}", data.read().first()))
        .finish()
}

#[get("/{uuid}")]
async fn quiz(
    web::Path(uuid): web::Path<String>,
    data: web::Data<RwLock<AppState>>,
) -> impl Responder {
    match Uuid::from_str(&uuid) {
        Ok(uuid) => match data.read().get(&uuid) {
            Some(quiz) => quiz.into_response(),
            None => Ok(HttpResponse::NotFound().finish()),
        },
        Err(_) => Ok(HttpResponse::NotFound().finish()),
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

    #[clap(short, long)]
    watch: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let lock_path = match args.lock {
        Some(lock_path) => lock_path,
        None => args.config.with_extension("lock"),
    };
    let data = web::Data::new(RwLock::new(
        Config::new(&args.config, &lock_path).into_state(),
    ));

    if args.watch {
        watch(&args.config, &lock_path, data.clone());
    }
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(quiz)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
