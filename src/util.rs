use rusqlite::{Connection, ToSql};

use crate::{
    error::Error,
    model::{Entry, Item},
};

pub fn easy_query(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
) -> Result<Vec<Item>, Error> {
    let mut statement = conn.prepare(&query)?;
    let results: Vec<Item> = statement
        .query_map(params, |row| {
            Ok(Item {
                title: row.get::<&str, String>("title").unwrap(),
                guid: row.get::<&str, String>("guid").unwrap(),
                pub_date: row.get::<&str, i64>("pub_date").ok(),
                enclosure: row.get::<&str, String>("enclosure").ok(),
                content: row.get::<&str, String>("content").ok(),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    Ok(results)
}

pub fn easy_query_entry(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
) -> Result<Vec<Entry>, Error> {
    let mut statement = conn.prepare(&query)?;
    let results: Vec<Entry> = statement
        .query_map(params, |row| {
            Ok(Entry {
                mentioned_title: row.get::<&str, String>("mentioned_title").unwrap(),
                mentioned_guid: row.get::<&str, String>("mentioned_guid").unwrap(),
                contained_episode: row.get::<&str, String>("contained_episode").unwrap(),
                contained_guid: row.get::<&str, String>("contained_guid").unwrap(),
                pub_date: row.get::<&str, i64>("pub_date").unwrap(),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    Ok(results)
}
