# sysk-forgets

This project downloads (`sync`s) the Stuff You Should Know RSS feed and stores it in a `search`able database. The user then `select`s an episode they heard mentioned, contained in the episode they heard it. Finally, the user `generate`s a RSS feed that can be imported into a podcast app as usual. 

Full Help Text:
```rust
sysk-forgets 0.0.1
Cassandra McCarthy <7596ff@gmail.com>
Reads the Stuff You Should Know RSS feed and stores it in a sqlite database

USAGE:
    sysk-forgets [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database <FILE>    

SUBCOMMANDS:
    generate    Generate an RSS feed
    help        Prints this message or the help of the given subcommand(s)
    search      Search for a term
    select      Select a term
    sync        Download feed and upsert into database
```

