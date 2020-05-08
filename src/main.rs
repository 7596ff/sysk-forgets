pub mod commands;
pub mod error;
pub mod model;
pub mod util;

use std::{fs, path::PathBuf};

use clap::{App, Arg};
use directories::BaseDirs;
use rusqlite::Connection;

use commands::*;
use error::Error;

const CREATE_DB: &'static str = include_str!("sql/create/db.sql");
const FEED_URL: &'static str = "https://feeds.megaphone.fm/stuffyoushouldknow";

fn main() -> Result<(), Error> {
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
        .subcommand(App::new("sync").about("Download feed and upsert into database"))
        .subcommand(
            App::new("search")
                .about("Search for a term")
                .arg(Arg::with_name("input").multiple(true)),
        )
        .subcommand(
            App::new("select")
                .about("Select a term")
                .arg(Arg::with_name("input").multiple(true)),
        )
        .subcommand(App::new("generate").about("Generate an RSS feed"));

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

    let conn = Connection::open(&database_location)?;

    // initialize the database if empty
    conn.execute_batch(&CREATE_DB)?;

    match matches.subcommand_name() {
        Some("sync") => sync::exec(&FEED_URL, conn),
        Some("search") => {
            if let Some(search_matches) = matches.subcommand_matches("search") {
                let search_text = match search_matches.values_of("input") {
                    Some(input) => input.collect::<Vec<&str>>().join(" "),
                    None => String::new(),
                };

                search::exec(search_text, conn)
            } else {
                Ok(())
            }
        }
        Some("select") => {
            if let Some(search_matches) = matches.subcommand_matches("select") {
                let search_text = match search_matches.values_of("input") {
                    Some(input) => input.collect::<Vec<&str>>().join(" "),
                    None => String::new(),
                };

                select::exec(search_text, conn)
            } else {
                Ok(())
            }
        }
        Some("generate") => generate::exec(conn),
        _ => Ok(()),
    }
}
