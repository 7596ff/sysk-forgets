use std::collections::HashMap;
use std::io::{self, Write};

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, ItemBuilder};
use rusqlite::{params, Connection};

use crate::model::{Entry, Item};

pub fn exec(conn: Connection) -> Result<()> {
    let now = Utc::now().timestamp();

    let mut stmt = conn.prepare("SELECT * FROM mentioned_items ORDER BY pub_date DESC;")?;
    let mut rows = stmt.query(params![])?;

    let mut entries = Vec::new();
    while let Some(row) = rows.next()? {
        let entry = Entry::from(row);
        if entry.pub_date < now {
            entries.push(entry);
        }
    }

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

    // add namespaces
    let mut namespaces: HashMap<String, String> = HashMap::new();

    namespaces.insert(
        "itunes".into(),
        "http://www.itunes.com/dtds/podcast-1.0.dtd".into(),
    );
    namespaces.insert(
        "googleplay".into(),
        "http://www.google.com/schemas/play-podcasts/1.0".into(),
    );
    namespaces.insert("atom".into(), "http://www.w3.org/2005/Atom".into());
    namespaces.insert("media".into(), "http://search.yahoo.com/mrss/".into());
    namespaces.insert(
        "content".into(),
        "http://purl.org/rss/1.0/modules/content/".into(),
    );

    channel.set_namespaces(namespaces);

    // get mentioned episode metadata, and add it to a list of items
    let mut items = Vec::new();
    for entry in entries {
        let mut stmt = conn.prepare("SELECT * FROM items WHERE guid = ?1 LIMIT 1;")?;
        let mut rows = stmt.query(params![entry.mentioned_guid])?;
        let mentioned = match rows.next()? {
            Some(item) => Item::from(item),
            None => continue,
        };

        let mentioned_pub_date = NaiveDateTime::from_timestamp(mentioned.pub_date, 0);
        let published = NaiveDateTime::from_timestamp(entry.pub_date, 0);

        let enclosure = EnclosureBuilder::default()
            .url(mentioned.enclosure)
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
                mentioned.content.to_string()
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
    channel.pretty_write_to(handle, b' ', 4)?;

    Ok(())
}
