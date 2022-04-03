use super::super::constants::BOOKS_TABLE;
use super::book::Book;
use super::filter::Filters;
use actix_web::web;

pub fn get_books_query(filter: web::Query<Filters>) -> String {
    let mut query = format!("Select * From {}", BOOKS_TABLE);
    if !!!filter.author.is_none() {
        query = format!(
            "{} where author='{}'",
            query,
            filter.author.as_ref().unwrap()
        );
    }
    if !!!filter.limit.is_none() {
        query = format!("{} limit {}", query, filter.limit.unwrap());
    }
    return query;
}

pub fn get_book_query() -> String {
    return format!("Select * From {} where id=?", BOOKS_TABLE);
}

pub fn create_book_query() -> String {
    return format!(
        "INSERT INTO {} (title, author) values (?, ?) RETURNING id",
        BOOKS_TABLE
    );
}

fn get_update_str(book: Book) -> String {
    let mut update_str = String::from("");

    if !!!book.title.is_none() {
        update_str.push_str("title='");
        update_str.push_str(book.title.unwrap().as_str());
        update_str.push_str("'");
    }
    if !!!book.author.is_none() {
        if !update_str.is_empty() {
            update_str.push_str(", ");
        }
        update_str.push_str("author='");
        update_str.push_str(book.author.unwrap().as_str());
        update_str.push_str("'");
    }
    update_str
}

pub fn update_book_query(book: Book) -> String {
    let fields = get_update_str(book);
    if fields.is_empty() {
        return fields;
    }
    return format!(
        "{}{}{}",
        format!("UPDATE {} SET ", BOOKS_TABLE),
        fields,
        " where id=?"
    );
}

pub fn delete_book_query() -> String {
    return format!("DELETE From {} where id=?", BOOKS_TABLE);
}
