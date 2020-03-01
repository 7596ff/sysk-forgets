[![license badge]][license link]

# sysk-forgets

This project downloads (`sync`s) the Stuff You Should Know RSS feed and stores it in a `search`able database. The user then `select`s an episode they heard mentioned, contained in the episode they heard it. Finally, the user `generate`s a RSS feed that can be imported into a podcast app as usual. 

Full help text:
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

Add this RSS feed to your podcast app to automatically recieve the episodes that I deem interesting to listen to: [https://croagunk.7596ff.com/sysk-forgets.xml](https://croagunk.7596ff.com/sysk-forgets.xml)

# License

sysk-forgets is in the public domain.

To the extent possible under law, Cassandra McCarthy <cassie@7596ff.com>
has waived all copyright and related or neighboring rights to this work.

[https://creativecommons.org/publicdomain/zero/1.0/](https://creativecommons.org/publicdomain/zero/1.0/)

[license badge]: https://img.shields.io/static/v1?label=license&message=public%20domain&color=7596ff&style=flat-square
[license link]: https://creativecommons.org/publicdomain/zero/1.0/
