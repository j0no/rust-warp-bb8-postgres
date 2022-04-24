# rust-warp-bb8-postgres

An example project to get warp and bb8_postgres working.

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new?template=https://railway.app/new/template?template=https%3A%2F%2Fgithub.com%2Fj0no%2Frust-warp-bb8-postgres)

## Features

- [warp](https://github.com/seanmonstar/warp)
- [bb8_postgres](https://docs.rs/bb8-postgres/latest/bb8_postgres/)

## How to run
1. Set .env varables
2. Run `cargo run`


## Notes
Envirment variable `DB_INFO` can be a database url or `key=value` diveded by spaces. 
The server returns `{"message": "Hello World" }` on `/` and  `{"message": "yo" }` on `/yo`.
