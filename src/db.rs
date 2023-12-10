use serde::Serialize;
use sqlx::FromRow;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Card {
    pub id: i64,
    pub title: String,
    pub list_id: i64,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct List {
    pub id: i64,
    pub title: String,
    pub cards_order: String,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub lists_order: String,
}

pub async fn init(url: impl Into<&str>) -> Result<SqlitePool, sqlx::Error> {
    let url = url.into();

    let new = if !Sqlite::database_exists(url).await.unwrap_or(false) {
        println!("Creating database {}", url);
        Sqlite::create_database(url).await?;
        println!("Create db success, running migrations");
        true
    } else {
        println!("Database already exists");
        false
    };

    let db = SqlitePool::connect(url).await?;

    if new {
        #[cfg(not(debug_assertions))]
        sqlx::migrate!().run(&db).await?;
        #[cfg(debug_assertions)]
        sqlx::migrate!("./migrations_debug/").run(&db).await?;

        println!("Migrations successful");
    }

    Ok(db)
}
