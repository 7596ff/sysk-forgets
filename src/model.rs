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
            title: row.get_unwrap::<usize, String>(0),
            pub_date: row.get_unwrap::<usize, i64>(1),
            itunes_author: row.get_unwrap::<usize, String>(2),
            itunes_image: row.get_unwrap::<usize, String>(3),
            itunes_subtitle: row.get_unwrap::<usize, String>(4),
            content: row.get_unwrap::<usize, String>(5),
            itunes_duration: row.get_unwrap::<usize, String>(6),
            guid: row.get_unwrap::<usize, String>(7),
            enclosure: row.get_unwrap::<usize, String>(8),
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
            mentioned_title: row.get_unwrap::<usize, String>(0),
            mentioned_guid: row.get_unwrap::<usize, String>(1),
            contained_episode: row.get_unwrap::<usize, String>(2),
            contained_guid: row.get_unwrap::<usize, String>(3),
            pub_date: row.get_unwrap::<usize, i64>(4),
        }
    }
}
