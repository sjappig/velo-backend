Vallila Elo Ranking Backend
===========================

Initial version was written in Vincit Cityhackfest 2017 within one day.

Requires postgresql instance running in localhost, with username `postgres` (no password) and database `velo_backend`.

## How to setup development environment

Install `diesel_cli` for database migrations:
```
cargo install diesel_cli
```

(If you're missing dependencies, try `brew install mysql-connector-c` (MacOS) or `apt-get install libmysqlclient-dev` (Debian))

Run migrations:
```
diesel migration run
```

Populate the database for the first time:
```
cargo run gamedata.txt playerdata.txt
```
