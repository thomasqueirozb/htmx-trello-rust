use std::collections::HashMap;

use actix_web::{web::Data, Result as AwResult};
use sqlx::QueryBuilder;

use crate::{
    db::{self, QueryId, QueryIds},
    models,
    util::{Helper, InIndexVector},
    AppState,
};

pub async fn board_data(state: Data<AppState>) -> AwResult<(String, Vec<models::List>)> {
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
