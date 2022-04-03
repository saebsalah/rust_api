use actix_web::{web, App, HttpServer};

mod authors;
mod books;
mod constants;
mod db;
mod env_var;
mod responses;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_pool = db::establish_connection().await.unwrap();
    let addr = env_var::get_addr();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn_pool.clone()))
            .configure(books::books::config_books)
            .configure(authors::authors::config_authors)
    })
    .bind(addr)?
    .run()
    .await
}
