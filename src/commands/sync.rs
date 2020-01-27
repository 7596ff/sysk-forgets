use chrono::DateTime;
use rss::Channel;
use rusqlite::{params, Connection};

use crate::error::Error;

const INSERT_ITEM: &'static str = include_str!("../sql/insert_item.sql");
const STRFTIME: &str = "%a, %d %b %Y %H:%M:%S %z";

pub fn exec(feed: &'static str, conn: Connection) -> Result<(), Error> {
    let channel = Channel::from_url(&feed).expect("could not fetch rss feed");

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

        conn.execute(
            &INSERT_ITEM,
            params![
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
            ],
        )?;
    }

    println!("Added or updated {} feed items.", channel.items().len());
    Ok(())
}
