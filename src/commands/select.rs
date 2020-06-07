use std::process::exit;

use anyhow::Result;
use chrono::{Datelike, NaiveDateTime, NaiveTime, Utc, Weekday};
use essentials::prompt;
use rusqlite::{params, Connection};

use crate::model::Item;

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

fn get_index_from_vec(results: &Vec<Item>) -> Result<usize> {
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

pub fn exec(mut search_text: String, conn: Connection) -> Result<()> {
    // prompt for a search if there is none
    if search_text.is_empty() {
        search_text = prompt("Please enter a mentioned episode name: ")?;
    }

    // search for a mentioned episode
    search_text = format!("%{}%", search_text.trim());

    let mut stmt = conn.prepare("SELECT * FROM items WHERE title LIKE ?1 COLLATE NOCASE;")?;
    let mut rows = stmt.query(params![search_text])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Item::from(row));
    }

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

    let mut statement = conn.prepare("SELECT * FROM items WHERE title LIKE ?1 COLLATE NOCASE;")?;
    let mut rows = statement.query(params![search_text])?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Item::from(row));
    }

    if results.len() == 0 {
        println!("No results found.");
        exit(1);
    }
    print_results(&results);

    // pick a contained result
    let index = get_index_from_vec(&results)?;
    let contained = &results[index - 1];

    // get the last published date from the mentioned feed
    let mut statement =
        conn.prepare("SELECT pub_date FROM mentioned_items ORDER BY pub_date DESC LIMIT 1;")?;
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
        "INSERT OR REPLACE INTO mentioned_items (
            mentioned_title, mentioned_guid, contained_episode,
            contained_guid, pub_date
        ) VALUES ( ?1, ?2, ?3, ?4, ?5 );",
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
