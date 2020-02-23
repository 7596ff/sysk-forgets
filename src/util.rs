use rusqlite::{Connection, ToSql};

use crate::{
    error::Error,
    model::Item,
};

pub fn easy_query(conn: &Connection, query: &str, params: &[&dyn ToSql]) -> Result<Vec<Item>, Error> {
    let mut statement = conn.prepare(&query)?;
    let results: Vec<Item> = statement
        .query_map(params, |row| {
            Ok(Item {
                title: row.get::<_, String>("title").unwrap(),
                guid: row.get::<_, String>("guid").unwrap(),
                pub_date: row.get::<_, i64>("pub_date").ok(),
                enclosure: row.get::<_, String>("enclosure").ok(),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    Ok(results)
}
