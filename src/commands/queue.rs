use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use colored::Colorize;
use rusqlite::{params, Connection};

use crate::model::Entry;

pub fn exec(conn: Connection) -> Result<()> {
    let now = Utc::now().timestamp();

    let mut stmt = conn.prepare("SELECT * FROM mentioned_items ORDER BY pub_date DESC;")?;
    let mut rows = stmt.query(params![])?;

    let mut entries = Vec::new();
    while let Some(row) = rows.next()? {
        entries.push(Entry::from(row));
    }

    for entry in entries {
        let pub_date = NaiveDateTime::from_timestamp(entry.pub_date, 0).format("%Y-%m-%d");
        let line = format!(
            "{} \"{}\" mentioned in \"{}\"",
            pub_date,
            entry.mentioned_title.purple().bold(),
            entry.contained_episode.green().bold(),
        );

        if now > entry.pub_date {
            println!("{}", line.dimmed());
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
