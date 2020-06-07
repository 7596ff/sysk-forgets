use anyhow::{Context, Result};
use chrono::DateTime;
use http::uri::Uri;
use isahc::prelude::*;
use rss::Channel;
use rusqlite::{params, Connection};

const STRFTIME: &str = "%a, %d %b %Y %H:%M:%S %z";

pub fn exec(feed: &'static str, conn: Connection) -> Result<()> {
    let response = isahc::get(Uri::from_static(feed))?.text()?;
    let data = response.as_bytes();

    let channel = Channel::read_from(data).context("could not read from rss feed")?;
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO items (
             title, pub_date, itunes_author, itunes_image, itunes_subtitle,
             itunes_summary, content, itunes_duration, guid, enclosure
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
    )?;

    for item in channel.items().iter() {
        let title = item.title().unwrap_or_default();
        let pub_date = item.pub_date().unwrap_or_default();
        let content = item.content().unwrap_or_default();
        let guid = item.guid().unwrap().value();
        let enclosure = item.enclosure().unwrap().url();

        let itunes = item.itunes_ext().unwrap();
        let author = itunes.author().unwrap_or_default();
        let image = itunes.image().unwrap_or_default();
        let subtitle = itunes.subtitle().unwrap_or_default();
        let summary = itunes.summary().unwrap_or_default();
        let duration = itunes.duration().unwrap_or_default();

        let pub_date = DateTime::parse_from_str(&pub_date, &STRFTIME)?;

        stmt.execute(params![
            title,
            pub_date.timestamp(),
            author,
            image,
            subtitle,
            summary,
            content,
            duration,
            guid,
            enclosure
        ])?;
    }

    println!("Added or updated {} feed items.", channel.items().len());
    Ok(())
}
