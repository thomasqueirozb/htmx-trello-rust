use crate::util::{Helper, InIndexVector};
use crate::{db, models};

use actix_web::Result as AwResult;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use sqlx::{FromRow, Pool, QueryBuilder};

type DB = sqlx::Sqlite;
type DBPool = Pool<DB>;

#[async_trait]
pub trait QueryId {
    async fn query_id(id: i64, db: &DBPool) -> AwResult<Self>
    where
        Self: Sized;
}

#[async_trait]
pub trait QueryIds {
    async fn query_ids(ids: &[i64], db: &DBPool) -> AwResult<Vec<Self>>
    where
        Self: Sized;
}

#[async_trait]
impl QueryId for models::BoardData {
    async fn query_id(id: i64, db: &DBPool) -> AwResult<Self> {
        Ok(
            sqlx::query_as!(db::Board, "SELECT * FROM boards where id = ?", id)
                .fetch_one(db)
                .await
                .ensure_data_type()?
                .into(),
        )
    }
}

#[async_trait]
impl QueryIds for models::ListData {
    async fn query_ids(ids: &[i64], db: &DBPool) -> AwResult<Vec<Self>> {
        Ok(QueryBuilder::new("SELECT * FROM lists WHERE id")
            .in_index_vector(ids)
            .build_query_as::<db::List>()
            .fetch_all(db)
            .await
            .ensure_data_type()?
            .into_iter()
            .map(Self::from)
            .collect())
    }
}

#[async_trait]
impl QueryId for models::ListData {
    async fn query_id(id: i64, db: &DBPool) -> AwResult<Self> {
        Ok(
            sqlx::query_as!(db::List, "SELECT * FROM lists WHERE id = ?", id)
                .fetch_one(db)
                .await
                .ensure_data_type()?
                .into(),
        )
    }
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Card {
    pub id: i64,
    pub title: String,
    pub list_id: i64,
}

#[async_trait]
impl QueryId for Card {
    async fn query_id(id: i64, db: &DBPool) -> AwResult<Self> {
        Ok(
            sqlx::query_as!(db::Card, "SELECT * FROM cards where id = ?", id)
                .fetch_one(db)
                .await
                .ensure_data_type()?,
        )
    }
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
