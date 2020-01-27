use std::{
    io::{self, Write},
    process,
};

use dialoguer::Input;
use rusqlite::{params, Connection, Row, ToSql};

use crate::error::Error;

const SELECT_NAME: &'static str = include_str!("../sql/select_name.sql");
const SELECT_LAST_EIGHT: &'static str = include_str!("../sql/select_last_eight.sql");

#[derive(Debug)]
struct Item {
    title: String,
    guid: String,
}

fn easy_select(conn: &Connection, query: &str, params: &[&dyn ToSql]) -> Result<Vec<Item>, Error> {
    let mut statement = conn.prepare(query)?;
    let results: Vec<Item> = statement
        .query_map(params, |row| {
            Ok(Item {
                title: row.get_unwrap::<usize, String>(0),
                guid: row.get_unwrap::<usize, String>(1),
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    Ok(results)
}

fn print_results(results: &Vec<Item>) {
    let mut counter = 1;
    for result in results {
        if counter == 1 {
            println!("");
        }

        println!("[{}] {}", counter, result.title);
        counter += 1;
    }
}

pub fn exec(search_text: String, conn: Connection) -> Result<(), Error> {
    println!("Searching for {}", search_text);
    let search_text = format!("%{}%", search_text);

    let results = easy_select(&conn, &SELECT_NAME, params![search_text])?;
    print_results(&results);

    if results.len() > 9 {
        println!("Too many results. Please narrow down your search.");
        return Ok(());
    }

    let index: usize = loop {
        let input = Input::<String>::new()
            .with_prompt("Please choose an old episode")
            .interact()?;

        if let Ok(index) = input.parse::<usize>() {
            if index < 1 || index > results.len() {
                println!("Please choose a number between 1 and {}.", results.len());
                continue;
            } else {
                break index;
            }
        }
    };

    let mentioned = &results[index - 1];

    let results = easy_select(&conn, &SELECT_LAST_EIGHT, params![])?;
    print_results(&results);

    let index: usize = loop {
        let input = Input::<String>::new()
            .with_prompt("Please choose a contained episode")
            .interact()?;

        if let Ok(index) = input.parse::<usize>() {
            if index < 1 || index > results.len() {
                println!("Please choose a number between 1 and {}.", results.len());
                continue;
            } else {
                break index;
            }
        }
    };

    let contained = &results[index - 1];

    println!("{:?}", mentioned);
    println!("{:?}", contained);

    Ok(())
}
