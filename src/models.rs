use crate::db;

#[derive(Debug)]
pub struct List {
    pub id: i64,
    pub title: String,
    pub cards: Vec<db::Card>,
}

impl List {
    pub fn id(&self) -> String {
        format!("list-{}", self.id)
    }
}

impl From<ListData> for List {
    fn from(source: ListData) -> Self {
        Self {
            id: source.id,
            title: source.title,
            cards: source.cards,
        }
    }
}

#[derive(Debug)]
pub struct ListData {
    pub id: i64,
    pub title: String,
    pub cards: Vec<db::Card>,
    pub cards_order: Vec<i64>,
}

impl From<db::List> for ListData {
    fn from(source: db::List) -> Self {
        let cards_order: Vec<i64> =
            serde_json::from_str(&source.cards_order).expect("Garbage in DB cards_order");
        Self {
            id: source.id,
            title: source.title,
            cards: Vec::with_capacity(cards_order.len()),
            cards_order,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub lists: Vec<List>,
}

#[derive(Debug)]
pub struct BoardData {
    pub id: i64,
    pub title: String,
    pub lists_order: Vec<i64>,
}

impl From<db::Board> for BoardData {
    fn from(source: db::Board) -> Self {
        Self {
            id: source.id,
            title: source.title,
            lists_order: serde_json::from_str(&source.lists_order)
                .expect("Garbage in DB lists_order"),
        }
    }
}
