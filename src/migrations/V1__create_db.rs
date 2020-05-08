use barrel::{
    backend::Sqlite,
    migration::Migration,
    types::{text, integer},
};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table_if_not_exists("items", |t| {
        t.add_column("title", text());
        t.add_column("pub_date", integer());
        t.add_column("itunes_author", text());
        t.add_column("itunes_image", text());
        t.add_column("itunes_subtitle", text());
        t.add_column("itunes_summary", text());
        t.add_column("content", text());
        t.add_column("itunes_duration", text());
        t.add_column("guid", text().primary(true));
        t.add_column("enclosure", text());
    });

    m.create_table_if_not_exists("mentioned_items", |t| {
        t.add_column("mentioned_title", text());
        t.add_column("mentioned_guid", text());
        t.add_column("contained_episode", text());
        t.add_column("contained_guid", text().primary(true));
        t.add_column("pub_date", integer());
    });

    m.make::<Sqlite>()
}
