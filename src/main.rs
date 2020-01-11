use std::{fs, path::PathBuf};

use clap::{App, Arg};
use directories::BaseDirs;
use rusqlite::Connection;

mod commands;
use commands::*;

const CREATE_DB: &'static str = include_str!("sql/create_db.sql");
const FEED_URL: &'static str = "https://feeds.megaphone.fm/stuffyoushouldknow";

fn main() {
    let app = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("FILE")
                .takes_value(true),
        )
        .subcommand(App::new("sync").about("Download feed and upsert into database"));

    let matches = app.get_matches();

    let database_location = match matches.value_of("database") {
        Some(d) => PathBuf::from(d),
        None => {
            let base_dirs = BaseDirs::new().expect("no data directory found in system");
            let mut path = base_dirs.data_dir().to_path_buf();

            path.push("sysk-forgets");
            if !path.is_dir() {
                fs::create_dir(&path)
                    .expect("failed to create directory inside local data directory");
            }

            path.push("db");
            path
        }
    };

    let conn = Connection::open(&database_location)
        .expect("failed to create connection to sqlite database");

    // initialize the database if empty
    conn.execute_batch(&CREATE_DB)
        .expect("failed to initialize database");

    match matches.subcommand_name() {
        Some("sync") => sync::exec(&FEED_URL, conn),
        _ => println!("none"), // i want to print clap's help here
    }
}
