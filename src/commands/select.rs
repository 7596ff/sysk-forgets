use std::io::{self, Write};

use rusqlite::{params, Connection, Row};

use crate::error::Error;

const SELECT_NAME: &'static str = include_str!("../sql/select_name.sql");
const SELECT_LAST_EIGHT: &'static str = include_str!("../sql/select_last_eight.sql");

struct Item {
    title: String,
    guid: String,
}

// fn map(row: Row) -> Result<Item, > {
//     Ok(Item {
//         title: row.get_unwrap::<usize, String>(0),
//         guid: row.get_unwrap::<usize, String>(1),
//     })
// }

pub fn exec(search_text: String, conn: Connection) -> Result<(), Error> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let mut statement = conn.prepare(&SELECT_NAME)?;
    let results: Vec<Item> = statement
        .query_map(params![search_text], |row| {
            Ok(Item {
                title: row.get_unwrap::<usize, String>(0),
                guid: row.get_unwrap::<usize, String>(1),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    if results.len() > 5 {
        println!("Too many results. Please narrow down your search.");
        return Ok(());
    }

    let mut counter = 1;
    println!("\n{} result(s) found.", results.len());
    for result in &results {
        println!("[{}] {}", counter, result.title);
        counter += 1;
    }

    print!("\nPlease choose a result: ");
    io::stdout().flush()?;

    let mut mentioned = String::new();
    io::stdin().read_line(&mut mentioned)?;
    let mentioned: usize = mentioned.trim().parse()?;
    let mentioned = &results[mentioned - 1];

    let mut statement = conn.prepare(&SELECT_LAST_EIGHT)?;
    let results: Vec<Item> = statement
        .query_map(params![], |row| {
            Ok(Item {
                title: row.get_unwrap::<usize, String>(0),
                guid: row.get_unwrap::<usize, String>(1),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    let mut counter = 1;
    for result in &results {
        if counter == 1 {
            println!("");
        }

        println!("[{}] {}", counter, result.title);
        counter += 1;
    }

    print!("\nPlease choose a contained episode: ");
    io::stdout().flush()?;

    let mut contained = String::new();
    io::stdin().read_line(&mut contained)?;
    let contained: usize = contained.trim().parse()?;
    let contained = &results[contained - 1];

    Ok(())
}
