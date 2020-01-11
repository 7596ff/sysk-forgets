use rss::{Channel, Item};
use rusqlite::{params, Connection};

const INSERT_ITEM: &'static str = include_str!("../sql/insert_item.sql");

pub fn exec(feed: &'static str, conn: Connection) {
    let channel = Channel::from_url(&feed).expect("could not fetch rss feed");

    for item in channel.items().iter() {
        let title = item.title().unwrap_or("");
        let pub_date = item.pub_date().unwrap_or("");
        let content = item.content().unwrap_or("");
        let guid = item.guid().unwrap().value();
        let enclosure = item.enclosure().unwrap().url();

        let itunes = item.itunes_ext().unwrap();
        let author = itunes.author().unwrap_or("");
        let image = itunes.image().unwrap_or("");
        let subtitle = itunes.subtitle().unwrap_or("");
        let summary = itunes.summary().unwrap_or("");
        let duration = itunes.duration().unwrap_or("");

        conn.execute(
            &INSERT_ITEM,
            params![
                title, pub_date, author, image, subtitle, summary, content, duration, guid,
                enclosure
            ],
        )
        .unwrap();
    }

    println!("Added or updated {} feed items.", channel.items().len());
}
