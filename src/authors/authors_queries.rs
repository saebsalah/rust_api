use super::super::constants::AUTHORS_TABLE;
use super::author::Author;
use super::filter::Filters;
use actix_web::web;

pub fn get_authors_query(filter: web::Query<Filters>) -> String {
    let mut query = format!("Select * From {}", AUTHORS_TABLE);
    if !!!filter.limit.is_none() {
        query = format!("{} limit {}", query, filter.limit.unwrap());
    }
    return query;
}

pub fn get_author_query() -> String {
    return format!("Select * From {} where id=?", AUTHORS_TABLE);
}

pub fn create_author_query() -> String {
    return format!(
        "INSERT INTO {} (name) values (?) RETURNING id",
        AUTHORS_TABLE
    );
}

fn get_update_str(author: Author) -> String {
    let mut update_str = String::from("");

    if !!!author.name.is_none() {
        update_str.push_str("name='");
        update_str.push_str(author.name.unwrap().as_str());
        update_str.push_str("'");
    }
    update_str
}

pub fn update_author_query(author: Author) -> String {
    let fields = get_update_str(author);
    if fields.is_empty() {
        return fields;
    }
    return format!(
        "{}{}{}",
        format!("UPDATE {} SET ", AUTHORS_TABLE),
        fields,
        " where id=?"
    );
}

pub fn delete_author_query() -> String {
    return format!("DELETE From {} where id=?", AUTHORS_TABLE);
}
