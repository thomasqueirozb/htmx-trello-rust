mod db;
mod html;
mod models;
mod util;

use crate::util::{Helper, InIndexVector};

use actix_files::Files;
use actix_web::web::Data;
use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::Markup;
use sqlx::{QueryBuilder, SqlitePool};
use std::collections::HashMap;
use std::io;

const DB_URL: &str = "sqlite://sqlite.db";

struct AppState {
    db: SqlitePool,
}

#[get("/")]
async fn index(state: Data<AppState>) -> AwResult<Markup> {
    let board: models::BoardData = sqlx::query_as!(db::Board, "SELECT * FROM boards where id=1")
        .fetch_one(&state.db)
        .await
        .ensure_data_type()?
        .into();

    let lists: Vec<models::ListData> = QueryBuilder::new("SELECT * FROM lists WHERE id")
        .in_index_vector(&board.lists_order)
        .build_query_as::<db::List>()
        .fetch_all(&state.db)
        .await
        .ensure_data_type()?
        .into_iter()
        .map(models::ListData::from)
        .collect();

    let cards: Vec<db::Card> = QueryBuilder::new("SELECT * FROM cards WHERE list_id")
        .in_index_vector(&board.lists_order)
        .build_query_as::<db::Card>()
        .fetch_all(&state.db)
        .await
        .ensure_data_type()?;

    let mut lists: HashMap<i64, models::List> = lists
        .into_iter()
        .map(|list| {
            (
                list.id,
                models::List {
                    list,
                    cards: Vec::new(),
                },
            )
        })
        .collect();
    for card in cards {
        let list_id = &card.list_id;
        if let Some(list) = lists.get_mut(list_id) {
            list.cards.push(card);
        } else {
            println!("Couldn't find {list_id}");
        }
    }

    let ordered_lists = board
        .lists_order
        .into_iter()
        .map(|idx| lists.remove(&idx).unwrap())
        .collect();

    Ok(html::base(board.title, ordered_lists))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db = db::init(DB_URL).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db.clone() }))
            .service(index)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
