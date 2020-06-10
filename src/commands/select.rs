use std::process::exit;

use anyhow::Result;
use chrono::{Datelike, NaiveDateTime, NaiveTime, Utc, Weekday};
use rusqlite::{params, Connection};

use crate::{
    model::{Entry, Item},
    util,
};

fn get_next_date(current: NaiveDateTime) -> NaiveDateTime {
    match current.weekday() {
        Weekday::Fri | Weekday::Sun | Weekday::Mon => current,
        _ => get_next_date(NaiveDateTime::new(current.date().succ(), current.time())),
    }
}

fn get_index_from_vec(results: &Vec<Item>) -> Result<usize> {
    loop {
        let input = essentials::prompt("Please choose an index: ")?;

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

pub fn exec(conn: Connection, mut search_text: String) -> Result<()> {
    let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

    // prompt for a search if there is none
    if search_text.is_empty() {
        search_text = essentials::prompt("Please enter a mentioned episode name: ")?;
    }

    // search for a mentioned episode
    search_text = format!("%{}%", search_text.trim());
    let results = util::search(&conn, search_text)?;
    if results.len() == 0 {
        println!("No results found.");
        exit(1);
    }
    util::print_items(&results);

    // pick a mentioned result
    let index = get_index_from_vec(&results)?;
    let mentioned = &results[index - 1];

    // prompt and search for a contained episode
    let search_text = format!(
        "%{}%",
        essentials::prompt("Please enter a contained episode name: ")?.trim()
    );
    let results = util::search(&conn, search_text)?;
    if results.len() == 0 {
        println!("No results found.");
        exit(1);
    }
    util::print_items(&results);

    // pick a contained result
    let index = get_index_from_vec(&results)?;
    let contained = &results[index - 1];

    // get the last published date from the mentioned feed
    let mut statement =
        conn.prepare("SELECT * FROM mentioned_items ORDER BY pub_date DESC LIMIT 1;")?;
    let mut rows = statement.query(params![])?;

    // pick a date that is after the last published date,
    // and after today,
    // and not on tue/wed/thur/sat
    let mut last_date = match rows.next()? {
        Some(latest) => NaiveDateTime::from_timestamp(Entry::from(latest).pub_date, 0),
        None => now,
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
