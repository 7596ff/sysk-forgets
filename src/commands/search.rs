use rusqlite::{params, Connection};

const SELECT_NAME: &'static str = include_str!("../sql/select_name.sql");

struct Item {
    title: String,
}

pub fn exec(search_text: String, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let mut statement = conn.prepare(&SELECT_NAME)?;
    let results: Vec<Result<Item, _>> = statement.query_map(params![search_text], |row| {
        Ok(Item {
            title: row.get_unwrap::<usize, String>(0),
        })
    })?.collect();

    println!("\n{} result(s) found.", results.len());
    for result in results {
        let result = result.unwrap();
        println!("{}", result.title);
    }

    Ok(())
}
