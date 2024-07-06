### DB

setup [diesel.rs](https://diesel.rs/guides/getting-started)

- `cargo install diesel_cli --no-default-features --features "postgres"`
- install [postgresql](https://www.postgresql.org/download/)
- `diesel setup`
- `diesel migration run`

### Fill .env

- `cp .env.example .env`
- get an api key [here](https://developer.hypixel.net/)
- edit .env file to match ur api key
- edit .env file to match ur pgsql logins

### Build

- `cargo build --release`

> u can run your app from here !
