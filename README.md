## Domain Driven Design and Rust

An example project of Domain Driven Design using Rust.

If you want to give it a try, you can follow these steps:

1. ensure that [docker-compose](https://docs.docker.com/compose/) is installed, and up-to-date.
2. ensure that [rust](https://www.rust-lang.org/learn/get-started) is installed, and up-to-date.
3. ensure that [sqlx cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install) is installed, and up-to-date.
4. run `git clone https://github.com/azjezz/ddd-rust`
5. run `cd ddd-rust`
6. run `docker-compose up -d`
7. run `sqlx database reset -y --database-url "postgres://main:main@127.0.0.1:5432/main"`
8. run `cargo run --release`
9. open [http://localhost:8080](http://localhost:8080) in your browser.
