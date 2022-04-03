use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};

use super::super::responses::CreateResponse;
use super::super::responses::CustomError;
use super::author::Author;
use super::authors_db;
use super::filter::Filters;

pub fn config_authors(cfg: &mut web::ServiceConfig) {
    cfg.service(get_authors)
        .service(get_author)
        .service(create_author)
        .service(update_author)
        .service(delete_author);
}

#[get("/authors")]
async fn get_authors(filter: web::Query<Filters>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = authors_db::get_authors(filter, pool).await;

    match r {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[get("/authors/{id}")]
async fn get_author(req: HttpRequest, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = authors_db::get_author(pool, req.match_info().query("id").parse().unwrap()).await;

    match r {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[post("/authors")]
async fn create_author(json: web::Json<Author>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = authors_db::create_author(pool, json.clone()).await;

    match r {
        Ok(r) => HttpResponse::Created().json(CreateResponse {
            id: r.last_insert_rowid(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[put("/authors/{id}")]
async fn update_author(
    req: HttpRequest,
    json: web::Json<Author>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let r = authors_db::update_author(
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

#[delete("/authors/{id}")]
async fn delete_author(req: HttpRequest, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let r = authors_db::delete_author(pool, req.match_info().query("id").parse().unwrap()).await;

    match r {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(e) => HttpResponse::InternalServerError().json(CustomError::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::*;
    use crate::authors::author::Author;
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
            name text
          );
        ",
            constants::AUTHORS_TABLE
        ))
        .execute(&sqlite_pool.clone())
        .await?;

        Ok(sqlite_pool)
    }

    #[actix_web::test]
    async fn test_get_authors() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_authors)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get().uri("/authors").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_authors_limit() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_authors)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/authors?limit=2")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_author)
                .service(super::create_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;
        let req = test::TestRequest::post()
            .set_json(Author {
                id: Some(1),
                name: Some("test1".to_string()),
            })
            .uri("/authors")
            .to_request();

        test::call_service(&app, req).await;

        let req = test::TestRequest::get().uri("/authors/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_author_internal_server_error() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::get_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::get().uri("/authors/0").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn test_create_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::create_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::post()
            .set_json(Author {
                id: Some(1),
                name: Some("test1".to_string()),
            })
            .uri("/authors")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_create_author_bad_request() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::create_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::post()
            .set_json("")
            .uri("/authors")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_update_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::update_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/authors/1")
            .set_json(Author {
                id: Some(1),
                name: Some("test1".to_string()),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_author_bad_request() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::update_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/authors/1")
            .set_json("")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_delete_author() {
        let conn_pool = establish_connection().await.unwrap();
        let app = test::init_service(
            App::new()
                .service(super::delete_author)
                .app_data(web::Data::new(conn_pool.clone())),
        )
        .await;

        let req = test::TestRequest::delete().uri("/authors/2").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
