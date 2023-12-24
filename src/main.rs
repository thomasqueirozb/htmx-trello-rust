mod board;
mod card;
mod db;
mod html;
mod models;
mod util;

use crate::board::board_data;
use crate::card::move_card;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::Markup;
use sqlx::SqlitePool;
use std::io;

const DB_URL: &str = "sqlite://sqlite.db";

struct AppState {
    db: SqlitePool,
}

#[get("/")]
async fn index(state: Data<AppState>) -> AwResult<Markup> {
    let board = board_data(state).await?;
    Ok(html::base(board.0, board.1))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db = db::init(DB_URL).await.unwrap();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .service(index)
            .service(move_card)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
