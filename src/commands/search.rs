use rusqlite::{params, Connection};

use crate::{error::Error, util::easy_query};

const SELECT_NAME: &'static str = include_str!("../sql/search/select_name.sql");

pub fn exec(search_text: String, conn: Connection) -> Result<(), Error> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let results = easy_query(&conn, &SELECT_NAME, params![search_text])?;

    println!("\n{} result(s) found.", results.len());
    for result in results {
        println!("{}", result.title);
    }

    Ok(())
}
