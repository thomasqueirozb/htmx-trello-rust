mod db;
mod html;
mod models;
mod util;

use crate::db::{QueryId, QueryIds};
use crate::util::{CustomError, Helper, InIndexVector, ParseIndexVector};

use actix_files::Files;
use actix_web::web::{self, Data};
use actix_web::{get, post, App, HttpServer, Result as AwResult};
use maud::Markup;
use serde::Deserialize;
use sqlx::{QueryBuilder, SqlitePool};
use std::collections::HashMap;
use std::io;

const DB_URL: &str = "sqlite://sqlite.db";

struct AppState {
    db: SqlitePool,
}

async fn board(state: Data<AppState>) -> AwResult<(String, Vec<models::List>)> {
    let board = models::BoardData::query_id(1, &state.db).await?;
    let lists = models::ListData::query_ids(&board.lists_order, &state.db).await?;

    let cards: Vec<db::Card> = QueryBuilder::new("SELECT * FROM cards WHERE list_id")
        .in_index_vector(&board.lists_order)
        .build_query_as::<db::Card>()
        .fetch_all(&state.db)
        .await
        .ensure_data_type()?;

    let mut lists: HashMap<i64, models::ListData> =
        lists.into_iter().map(|list| (list.id, list)).collect();

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
        .map(|mut list| {
            list.cards = list
                .cards_order
                .iter()
                .filter_map(|&idx| list.cards.iter().find(|&card| card.id == idx))
                .cloned()
                .collect();
            list
        })
        .map(models::List::from)
        .collect();

    Ok((board.title, ordered_lists))
}

#[get("/")]
async fn index(state: Data<AppState>) -> AwResult<Markup> {
    let board = board(state).await?;
    Ok(html::base(board.0, board.1))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct MoveCard {
    card_id: i64,
    to_list_id: i64,
    new_position: i64,
}

#[post("/move/card")]
async fn move_card(
    state: Data<AppState>,
    web::Form(form): web::Form<MoveCard>,
) -> AwResult<Markup> {
    let MoveCard {
        card_id,
        to_list_id,
        new_position,
    } = form;

    #[derive(Debug)]
    struct Query {
        id: i64,
        cards_order: String,
    }

    let mut query: Vec<Query> = sqlx::query_as!(
        Query,
        "SELECT id, cards_order FROM lists WHERE id IN
        (?, (SELECT list_id FROM cards WHERE id = ?));",
        to_list_id,
        card_id,
    )
    .fetch_all(&state.db)
    .await
    .ensure_query_success()?;

    let query_len = query.len();
    if !(query_len == 1 || query_len == 2) {
        dbg!(&query);
        return Err(CustomError::InsufficientItemsReturned(format!(
            "Move card query not 1 or 2 ({query_len}): to list id: {to_list_id}, card id {card_id}"
        ))
        .into());
    }
    let actual_position = |new_position: i64, len: usize| -> usize {
        if new_position < 0 {
            len
        } else {
            new_position as usize
        }
    };

    if query_len == 1 {
        let list = query.pop().unwrap();

        let mut cards_order = list.cards_order.parse_index_vector()?;
        let Some(cards_order_card_id) = cards_order.iter().position(|&id| id == card_id) else {
            return Err(
                CustomError::Other("Card not found in from positions list".to_string()).into(),
            );
        };
        cards_order.remove(cards_order_card_id);

        cards_order.insert(actual_position(new_position, cards_order.len()), card_id);
        let cards_order = serde_json::to_string(&cards_order).unwrap();

        sqlx::query!(
            "BEGIN TRANSACTION;
            UPDATE lists SET cards_order = ? WHERE id = ?;
            UPDATE cards SET list_id = ? WHERE id = ?;
            COMMIT;",
            cards_order,
            list.id,
            list.id,
            card_id,
        )
        .execute(&state.db)
        .await
        .ensure_query_success()?;
    } else {
        let popped = query.pop().unwrap();

        let (to_list, from_list) = if popped.id == to_list_id {
            (popped, query.pop().unwrap())
        } else {
            (query.pop().unwrap(), popped)
        };

        let mut from_cards_order = from_list.cards_order.parse_index_vector()?;
        let Some(from_cards_order_card_id) = from_cards_order.iter().position(|&id| id == card_id)
        else {
            return Err(
                CustomError::Other("Card not found in from positions list".to_string()).into(),
            );
        };
        from_cards_order.remove(from_cards_order_card_id);
        let from_cards_order = serde_json::to_string(&from_cards_order).unwrap();

        let mut to_cards_order = to_list.cards_order.parse_index_vector()?;
        to_cards_order.insert(actual_position(new_position, to_cards_order.len()), card_id);
        let to_cards_order = serde_json::to_string(&to_cards_order).unwrap();

        sqlx::query!(
            "BEGIN TRANSACTION;
            UPDATE lists SET cards_order = ? WHERE id = ?;
            UPDATE lists SET cards_order = ? WHERE id = ?;
            UPDATE cards SET list_id = ? WHERE id = ?;
            COMMIT;",
            from_cards_order,
            from_list.id,
            to_cards_order,
            to_list.id,
            to_list.id,
            card_id,
        )
        .execute(&state.db)
        .await
        .ensure_query_success()?;
    }

    let board = board(state).await?.1;

    Ok(html::make_board(board))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db = db::init(DB_URL).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db.clone() }))
            .service(index)
            .service(move_card)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
