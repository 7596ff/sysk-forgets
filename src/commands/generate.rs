use std::io::{self, Write};

use chrono::{NaiveDateTime, Utc};
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, Item as RssItem, ItemBuilder};
use rusqlite::{params, Connection};

use crate::{
    error::Error,
    util::{easy_query, easy_query_entry},
};

const SELECT_ENTRIES: &'static str = include_str!("../sql/generate/select_entries.sql");
const SELECT_ITEM_BY_GUID: &'static str = include_str!("../sql/generate/select_item_by_guid.sql");

pub fn exec(conn: Connection) -> Result<(), Error> {
    let now = Utc::now().timestamp();

    // get list of entries from mentioned_items and filter out any future events
    let entries = easy_query_entry(&conn, &SELECT_ENTRIES, params![])?;
    let entries = entries.iter().filter(|x| x.pub_date < now);

    // form the rss feed
    let mut channel = ChannelBuilder::default()
        .title("SYSK Forgets")
        .link("https://gitlab.com/7596ff/sysk-forgets/")
        .description(
            "A feed of all the episodes that Josh and Chuck mention, republished on off-days",
        )
        .language("en".to_string())
        .build()
        .unwrap();

    // get mentioned episode metadata, and add it to a list of items
    let mut items: Vec<RssItem> = vec![];
    for entry in entries {
        let mentioned =
            easy_query(&conn, &SELECT_ITEM_BY_GUID, params![entry.mentioned_guid])?.remove(0);
        let mentioned_pub_date = NaiveDateTime::from_timestamp(mentioned.pub_date.unwrap(), 0);

        let published = NaiveDateTime::from_timestamp(entry.pub_date, 0);

        let enclosure = EnclosureBuilder::default()
            .url(mentioned.enclosure.unwrap())
            .length("0")
            .mime_type("audio/mpeg")
            .build()
            .unwrap();

        let guid = GuidBuilder::default()
            .value(published.timestamp().to_string())
            .build()
            .unwrap();

        let item = ItemBuilder::default()
            .title(format!("SYSK Forgets: {}", entry.mentioned_title))
            .enclosure(enclosure)
            .guid(guid)
            .pub_date(
                published
                    .format("%a, %d %b %Y %H:%M:%S -0000")
                    .to_string(),
            )
            .content(format!(
                "<p>This episode was originally published on {}. It was mentioned in the episode \"{}\".</p> {}",
                mentioned_pub_date.format("%Y-%m-%d").to_string(),
                entry.contained_episode,
                mentioned.content.unwrap().to_string()
            ))
            .build()
            .unwrap();

        items.push(item);
    }

    channel.set_items(items);

    // write to stdout
    let out = io::stdout();
    let mut handle = out.lock();

    write!(handle, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    #[rustfmt::skip]
    write!(handle, "{} {} {} {} {} {}\n",
        "<rss version=\"2.0\"",
        "xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\"",
        "xmlns:googleplay=\"http://www.google.com/schemas/play-podcasts/1.0\"",
        "xmlns:atom=\"http://www.w3.org/2005/Atom\"",
        "xmlns:media=\"http://search.yahoo.com/mrss/\"",
        "xmlns:content=\"http://purl.org/rss/1.0/modules/content/\">"
    )?;

    // need to remove the first line of pretty written rss, because it contains an extra <rss> tag
    let buffer: Vec<u8> = vec![];
    let buffer = channel.pretty_write_to(buffer, b' ', 4)?;
    let buffer = String::from_utf8(buffer)?;

    let mut buffer = buffer.lines();
    let _ = buffer.next();
    let buffer = buffer.collect::<Vec<&str>>().join("\n");

    write!(handle, "{}", buffer)?;

    Ok(())
}
