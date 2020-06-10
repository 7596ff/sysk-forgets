use anyhow::Result;
use rusqlite::Connection;

use crate::util;

pub fn exec(conn: Connection, search_text: String) -> Result<()> {
    println!("Searching for \"{}\"", search_text);
    let search_text = format!("%{}%", search_text);
    let results = util::search(&conn, search_text)?;

    println!("{} result(s) found.", results.len());
    util::print_items(&results);

    Ok(())
}
