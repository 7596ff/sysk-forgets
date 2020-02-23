#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub guid: String,
    pub pub_date: Option<i64>,
    pub enclosure: Option<String>,
}

