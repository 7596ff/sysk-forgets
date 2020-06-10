mod commands;
mod migrations;
mod model;
mod util;

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::{App, Arg};
use directories::BaseDirs;
use rusqlite::Connection;

const FEED_URL: &'static str = "https://feeds.megaphone.fm/stuffyoushouldknow";

fn main() -> Result<()> {
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
        .subcommand(App::new("generate").about("Generate an RSS feed"))
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

    let mut conn =
        Connection::open(&database_location).context("failed to open database at location")?;

    migrations::runner()
        .run(&mut conn)
        .context("failed to run migrations")?;

    match matches.subcommand_name() {
        Some("generate") => commands::generate(conn),
        Some("search") => match matches.subcommand_matches("search") {
            Some(matches) => commands::search(
                conn,
                match matches.values_of("input") {
                    Some(input) => input.collect::<Vec<&str>>().join(" "),
                    None => String::new(),
                },
            ),
            None => Ok(()),
        },
        Some("select") => match matches.subcommand_matches("select") {
            Some(matches) => commands::select(
                conn,
                match matches.values_of("input") {
                    Some(input) => input.collect::<Vec<&str>>().join(" "),
                    None => String::new(),
                },
            ),
            None => Ok(()),
        },
        Some("sync") => commands::sync(conn, &FEED_URL),
        _ => Ok(()),
    }
}
