use std::process::exit;

use anyhow::Error;
use chrono::{Datelike, NaiveDateTime, NaiveTime, Utc, Weekday};
use essentials::prompt;
use rusqlite::{params, Connection};

use crate::{model::Item, util::easy_query};

const SELECT_NAME: &'static str = include_str!("../sql/search/select_name.sql");
const SELECT_MENTIONED_LAST_DATE: &'static str =
    include_str!("../sql/select/mentioned_last_date.sql");
const INSERT_MENTIONED: &'static str = include_str!("../sql/select/insert_mentioned.sql");

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

fn get_index_from_vec(results: &Vec<Item>) -> Result<usize, Error> {
    loop {
        let input = prompt("Please choose an index: ")?;

        if let Ok(index) = input.trim().parse::<usize>() {
            if index < 1 || index > results.len() + 1 {
                println!("Please choose a number between 1 and {}.", results.len());
                continue;
            } else {
                break Ok(index);
            }
        }
    }
}

pub fn exec(mut search_text: String, conn: Connection) -> Result<(), Error> {
    // prompt for a search if there is none
    if search_text.is_empty() {
        search_text = prompt("Please enter a mentioned episode name: ")?;
    }

    // search for a mentioned episode
    search_text = format!("%{}%", search_text.trim());
    let results = easy_query(&conn, &SELECT_NAME, params![search_text])?;
    if results.len() == 0 {
        println!("No results found.");
        exit(1);
    }
    print_results(&results);

    // pick a mentioned result
    let index = get_index_from_vec(&results)?;
    let mentioned = &results[index - 1];

    // prompt and search for a contained episode
    search_text = prompt("Please enter a contained episode name: ")?;
    search_text = format!("%{}%", search_text.trim());
    let results = easy_query(&conn, &SELECT_NAME, params![search_text])?;
    if results.len() == 0 {
        println!("No results found.");
        exit(1);
    }
    print_results(&results);

    // pick a contained result
    let index = get_index_from_vec(&results)?;
    let contained = &results[index - 1];

    // get the last published date from the mentioned feed
    let mut statement = conn.prepare(&SELECT_MENTIONED_LAST_DATE)?;
    let results = statement.query_map(params![], |row| Ok(row.get::<usize, i64>(0)))?;
    let results: Vec<i64> = results.map(|i| i.unwrap().unwrap()).collect();

    // pick a date that is after the last published date,
    // and after today,
    // and not on tue/wed/thur/sat
    let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let mut last_date = if results.is_empty() {
        now
    } else {
        NaiveDateTime::from_timestamp(*results.first().unwrap(), 0)
    };

    if last_date < now {
        last_date = now;
    }

    let next_date = get_next_date(NaiveDateTime::new(
        last_date.date().succ(),
        NaiveTime::from_hms(10, 0, 0),
    ));

    // insert into database
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

    println!(
        "\nInserted episode \"{}\", which was mentioned in episode \"{}\", to be published on {}",
        mentioned.title, contained.title, next_date
    );

    Ok(())
}
