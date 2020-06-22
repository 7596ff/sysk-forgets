use rusqlite::Row;

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub pub_date: i64,
    pub itunes_author: String,
    pub itunes_image: String,
    pub itunes_subtitle: String,
    pub content: String,
    pub itunes_duration: String,
    pub guid: String,
    pub enclosure: String,
}

impl From<&Row<'_>> for Item {
    fn from(row: &Row) -> Self {
        Item {
            title: row.get_unwrap::<&str, String>("title"),
            pub_date: row.get_unwrap::<&str, i64>("pub_date"),
            itunes_author: row.get_unwrap::<&str, String>("itunes_author"),
            itunes_image: row.get_unwrap::<&str, String>("itunes_image"),
            itunes_subtitle: row.get_unwrap::<&str, String>("itunes_subtitle"),
            content: row.get_unwrap::<&str, String>("content"),
            itunes_duration: row.get_unwrap::<&str, String>("itunes_duration"),
            guid: row.get_unwrap::<&str, String>("guid"),
            enclosure: row.get_unwrap::<&str, String>("enclosure"),
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub mentioned_title: String,
    pub mentioned_guid: String,
    pub contained_episode: String,
    pub contained_guid: String,
    pub pub_date: i64,
}

impl From<&Row<'_>> for Entry {
    fn from(row: &Row) -> Self {
        Entry {
            mentioned_title: row.get_unwrap::<&str, String>("mentioned_title"),
            mentioned_guid: row.get_unwrap::<&str, String>("mentioned_guid"),
            contained_episode: row.get_unwrap::<&str, String>("contained_episode"),
            contained_guid: row.get_unwrap::<&str, String>("contained_guid"),
            pub_date: row.get_unwrap::<&str, i64>("pub_date"),
        }
    }
}
