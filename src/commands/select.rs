use chrono::{Datelike, NaiveDateTime, NaiveTime, Utc, Weekday};
use dialoguer::Input;
use rusqlite::{params, Connection, ToSql};

use crate::{
    error::Error,
    model::Item,
    util::easy_query,
};

const SELECT_NAME: &'static str = include_str!("../sql/select_name.sql");
const SELECT_LAST_TEN: &'static str = include_str!("../sql/select_last_ten.sql");
const SELECT_MENTIONED_LAST_DATE: &'static str =
    include_str!("../sql/select_mentioned_last_date.sql");
const INSERT_MENTIONED: &'static str = include_str!("../sql/insert_mentioned.sql");

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

fn get_next_date(current: NaiveDateTime) -> NaiveDateTime {
    match current.weekday() {
        Weekday::Fri | Weekday::Sun | Weekday::Mon => current,
        _ => get_next_date(NaiveDateTime::new(current.date().succ(), current.time())),
    }
}

pub fn exec(search_text: String, conn: Connection) -> Result<(), Error> {
    println!("Searching for \"{}\"", search_text);
    let search_text = format!("%{}%", search_text);

    let results = easy_query(&conn, &SELECT_NAME, params![search_text])?;
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

    let results = easy_query(&conn, &SELECT_LAST_TEN, params![])?;
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

    // get the last published date from the mentioned feed
    let mut statement = conn.prepare(&SELECT_MENTIONED_LAST_DATE)?;
    let results = statement.query_map(params![], |row| Ok(row.get::<usize, i64>(0)))?;
    let results: Vec<i64> = results.map(|i| i.unwrap().unwrap()).collect();

    let last_date = if results.is_empty() {
        NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
    } else {
        NaiveDateTime::from_timestamp(*results.first().unwrap(), 0)
    };

    let next_date = get_next_date(NaiveDateTime::new(
        last_date.date().succ(),
        NaiveTime::from_hms(10, 0, 0),
    ));

    conn.execute(
        &INSERT_MENTIONED,
        params![
            mentioned.title,
            mentioned.guid,
            contained.title,
            contained.guid,
            next_date.timestamp()
        ],
    )?;

    println!("\nInserted episode \"{}\", which was mentioned in episode \"{}\", to be published on {}", mentioned.title, contained.title, next_date);

    Ok(())
}
