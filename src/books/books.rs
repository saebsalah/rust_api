use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};

use super::super::responses::CreateResponse;
use super::super::responses::CustomError;
use super::book::Book;
use super::books_db;
use super::filter::Filters;

pub fn config_books(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books)
        .service(get_book)
        .service(create_book)
        .service(update_book)
        .service(delete_book);
}

#[get("/books")]
async fn get_books(filter: web::Query<Filters>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = books_db::get_books(filter, pool).await;

    match r {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[get("/books/{id}")]
async fn get_book(req: HttpRequest, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = books_db::get_book(pool, req.match_info().query("id").parse().unwrap()).await;

    match r {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[post("/books")]
async fn create_book(json: web::Json<Book>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = books_db::create_book(pool, json.clone()).await;

    match r {
        Ok(r) => HttpResponse::Created().json(CreateResponse {
            id: r.last_insert_rowid(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[put("/books/{id}")]
async fn update_book(
    req: HttpRequest,
    json: web::Json<Book>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let r = books_db::update_book(
        json.clone(),
        pool,
        req.match_info().query("id").parse().unwrap(),
    )
    .await;

    match r {
        Ok(_) => HttpResponse::Ok().json("Updated"),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[delete("/books/{id}")]
async fn delete_book(req: HttpRequest, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = books_db::delete_book(pool, req.match_info().query("id").parse().unwrap()).await;

    match r {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::*;
    use crate::books::book::Book;
    use actix_web::{
        http::{self},
        test,
    };
    use sqlx::{
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
        Error, Pool, Sqlite,
    };
    use std::str::FromStr;
    const DATABASE_TEST_URL: &str = "sqlite://db_test.sqlite";

    pub async fn establish_connection() -> Result<Pool<Sqlite>, Error> {
        let connection_options =
            SqliteConnectOptions::from_str(DATABASE_TEST_URL)?.create_if_missing(true);

        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(constants::POOL_MAX_CONNECTIONS)
            .connect_with(connection_options)
            .await?;

        sqlx::query(&format!(
            "
        CREATE TABLE IF NOT EXISTS {} (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          title text,
          author text
        );
        ",
            constants::BOOKS_TABLE
        ))
        .execute(&sqlite_pool.clone())
        .await?;

        Ok(sqlite_pool)
    }

    #[actix_web::test]
    async fn test_get_books() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_books)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get().uri("/books").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_books_limit() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_books)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get().uri("/books?limit=2").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_books_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_books)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/books?author=saeb")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_books_limit_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_books)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/books?author=saeb&limit=1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_book() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_book)
                .service(super::create_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::post()
            .set_json(Book {
                id: Some(1),
                title: Some("test1".to_string()),
                author: Some("test1".to_string()),
            })
            .uri("/books")
            .to_request();

        test::call_service(&app, req).await;

        let req = test::TestRequest::get().uri("/books/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_book_internal_server_error() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get().uri("/books/0").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn test_create_book() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::create_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::post()
            .set_json(Book {
                id: Some(1),
                title: Some("test1".to_string()),
                author: Some("test1".to_string()),
            })
            .uri("/books")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_create_book_bad_request() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::create_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::post()
            .set_json("")
            .uri("/books")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_update_book() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::update_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/books/1")
            .set_json(Book {
                id: Some(1),
                title: Some("test1".to_string()),
                author: Some("test1".to_string()),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_book_bad_request() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::update_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/books/1")
            .set_json("")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_delete_book() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::delete_book)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::delete().uri("/books/2").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
