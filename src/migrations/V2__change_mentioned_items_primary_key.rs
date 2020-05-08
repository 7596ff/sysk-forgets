use barrel::{
    backend::Sqlite,
    migration::Migration,
    types::{text, integer},
};

pub fn before() -> String {
    let mut m = Migration::new();

    m.create_table_if_not_exists("mentioned_items_copy", |t| {
        t.add_column("mentioned_title", text());
        t.add_column("mentioned_guid", text());
        t.add_column("contained_episode", text());
        t.add_column("contained_guid", text());
        t.add_column("pub_date", integer().primary(true));
    });

    m.make::<Sqlite>()
}

pub fn during() -> String {
    "INSERT INTO mentioned_items_copy (
        mentioned_title, mentioned_guid, contained_episode, contained_guid, pub_date
    ) SELECT
        mentioned_title, mentioned_guid, contained_episode, contained_guid, pub_date
    FROM mentioned_items;".into()
}

pub fn after() -> String {
    let mut m = Migration::new();

    m.drop_table("mentioned_items");
    m.rename_table("mentioned_items_copy", "mentioned_items");

    m.make::<Sqlite>()
}

pub fn migration() -> String {
    format!("{} {} {}", before(), during(), after())
}

