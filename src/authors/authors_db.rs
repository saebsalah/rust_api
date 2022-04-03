use super::author::Author;
use super::authors_queries;
use super::filter::Filters;
use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, Error, Pool, Sqlite};

pub async fn get_authors(
    filter: web::Query<Filters>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Vec<Author>, Error> {
    let query = authors_queries::get_authors_query(filter);
    let r = sqlx::query_as::<_, Author>(&query)
        .fetch_all(pool.get_ref())
        .await;

    return r;
}

pub async fn get_author(pool: web::Data<Pool<Sqlite>>, author_id: i64) -> Result<Author, Error> {
    let query = authors_queries::get_author_query();
    let r = sqlx::query_as::<_, Author>(&query)
        .bind(author_id)
        .fetch_one(pool.get_ref())
        .await;

    return r;
}

pub async fn create_author(
    pool: web::Data<Pool<Sqlite>>,
    author: Author,
) -> Result<SqliteQueryResult, Error> {
    let query = authors_queries::create_author_query();
    let r = sqlx::query(&query)
        .bind(author.name)
        .execute(pool.get_ref())
        .await;

    return r;
}

pub async fn update_author(
    author: Author,
    pool: web::Data<Pool<Sqlite>>,
    author_id: i64,
) -> Result<SqliteQueryResult, Error> {
    let query = authors_queries::update_author_query(author);
    let r = sqlx::query(&query)
        .bind(author_id)
        .execute(pool.get_ref())
        .await;

    return r;
}

pub async fn delete_author(
    pool: web::Data<Pool<Sqlite>>,
    author_id: i64,
) -> Result<SqliteQueryResult, Error> {
    let query = authors_queries::delete_author_query();
    let r = sqlx::query(&query)
        .bind(author_id)
        .execute(pool.get_ref())
        .await;

    return r;
}
