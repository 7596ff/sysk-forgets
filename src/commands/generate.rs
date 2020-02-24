use chrono::{NaiveDateTime, Utc};
use rss::{ChannelBuilder, EnclosureBuilder, Item as RssItem, ItemBuilder};
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
        .description(
            "A feed of all the episodes that Josh and Chuck mention, republished on off-days",
        )
        .build()
        .unwrap();

    // get mentioned episode metadata, and add it to a list of items
    let mut items: Vec<RssItem> = vec![];
    for entry in entries {
        let mentioned = easy_query(&conn, &SELECT_ITEM_BY_GUID, params![entry.mentioned_guid])?;
        let contained = easy_query(&conn, &SELECT_ITEM_BY_GUID, params![entry.contained_guid])?;
        let mentioned = &mentioned[0];
        let contained = &contained[0];

        let originally_published = NaiveDateTime::from_timestamp(contained.pub_date.unwrap(), 0)
            .format("%Y-%m-%d %H:%M:%S");
        let enclosure = EnclosureBuilder::default()
            .url(mentioned.enclosure.as_ref().unwrap())
            .build()
            .unwrap();

        let item = ItemBuilder::default()
            .title(format!("SYSK Forgets: {}", entry.mentioned_title))
            .enclosure(enclosure)
            .content(format!(
                    "<p>This episode was originally published on {}.</p> {}",
                    originally_published,
                    mentioned.content.as_ref().unwrap().to_string()))
            .build()
            .unwrap();

        items.push(item);
    }

    channel.set_items(items);

    channel.pretty_write_to(std::io::stdout(), b' ', 4)?;

    Ok(())
}
