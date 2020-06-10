use anyhow::Result;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection};

use crate::model::Item;

pub fn search(conn: &Connection, search_text: String) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM items WHERE title LIKE ?1
        COLLATE NOCASE ORDER BY pub_date DESC;",
    )?;

    let mut rows = stmt.query(params![search_text])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Item::from(row));
    }

    Ok(results)
}

pub fn print_items(items: &Vec<Item>) {
    let mut count = 1;
    for item in items {
        let pub_date = NaiveDateTime::from_timestamp(item.pub_date, 0);
        println!(
            "{} [{:>4}] {}",
            pub_date.format("%Y-%m-%d"),
            count,
            item.title
        );
        count += 1;
    }
}
