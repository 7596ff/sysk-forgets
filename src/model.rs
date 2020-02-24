#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub guid: String,
    pub pub_date: Option<i64>,
    pub enclosure: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct Entry {
    pub mentioned_title: String,
    pub mentioned_guid: String,
    pub contained_episode: String,
    pub contained_guid: String,
    pub pub_date: i64,
}
