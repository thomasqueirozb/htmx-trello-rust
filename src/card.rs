use actix_web::web::{self, Data};
use actix_web::{post, Result as AwResult};
use maud::Markup;
use serde::Deserialize;

use crate::board::board_data;
use crate::util::{CustomError, Helper, ParseIndexVector};
use crate::{html, AppState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct MoveCard {
    card_id: i64,
    to_list_id: i64,
    new_position: i64,
}

#[post("/move")]
async fn move_(state: Data<AppState>, web::Form(form): web::Form<MoveCard>) -> AwResult<Markup> {
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

    let board = board_data(state).await?.1;

    Ok(html::make_board(board))
}

pub fn service() -> actix_web::Scope {
    web::scope("/card").service(move_)
}
