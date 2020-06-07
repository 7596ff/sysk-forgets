use anyhow::Result;
use rusqlite::{params, Connection};

use crate::util::easy_query;

pub fn exec(search_text: String, conn: Connection) -> Result<()> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let results = easy_query(
        &conn,
        "SELECT title, guid FROM items WHERE LIKE ?1 COLLATE NOCASE",
        params![search_text],
    )?;

    println!("\n{} result(s) found.", results.len());
    for result in results {
        println!("{}", result.title);
    }

    Ok(())
}
