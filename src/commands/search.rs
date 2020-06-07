use anyhow::Result;
use rusqlite::{params, Connection};

use crate::model::Item;

pub fn exec(search_text: String, conn: Connection) -> Result<()> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let mut stmt = conn.prepare("SELECT title, guid FROM items WHERE LIKE ?1 COLLATE NOCASE;")?;
    let mut rows = stmt.query(params![search_text])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Item::from(row));
    }

    println!("\n{} result(s) found.", results.len());
    for result in results {
        println!("{}", result.title);
    }

    Ok(())
}
