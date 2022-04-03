use super::book::Book;
use super::books_queries;
use super::filter::Filters;
use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, Error, Pool, Sqlite};

pub async fn get_books(
    filter: web::Query<Filters>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Vec<Book>, Error> {
    let query = books_queries::get_books_query(filter);

    let r = sqlx::query_as::<_, Book>(&query)
        .fetch_all(pool.get_ref())
        .await;

    return r;
}

pub async fn get_book(pool: web::Data<Pool<Sqlite>>, book_id: i64) -> Result<Book, Error> {
    let query = books_queries::get_book_query();
    let r = sqlx::query_as::<_, Book>(&query)
        .bind(book_id)
        .fetch_one(pool.get_ref())
        .await;

    return r;
}

pub async fn create_book(
    pool: web::Data<Pool<Sqlite>>,
    book: Book,
) -> Result<SqliteQueryResult, Error> {
    let query = books_queries::create_book_query();
    let r = sqlx::query(&query)
        .bind(book.title)
        .bind(book.author)
        .execute(pool.get_ref())
        .await;

    return r;
}

pub async fn update_book(
    book: Book,
    pool: web::Data<Pool<Sqlite>>,
    book_id: i64,
) -> Result<SqliteQueryResult, Error> {
    let query = books_queries::update_book_query(book);
    let r = sqlx::query(&query)
        .bind(book_id)
        .execute(pool.get_ref())
        .await;

    return r;
}

pub async fn delete_book(
    pool: web::Data<Pool<Sqlite>>,
    book_id: i64,
) -> Result<SqliteQueryResult, Error> {
    let query = books_queries::delete_book_query();
    let r = sqlx::query(&query)
        .bind(book_id)
        .execute(pool.get_ref())
        .await;

    return r;
}
