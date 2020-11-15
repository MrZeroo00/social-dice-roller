# Social Dice Roller

## Development

1. [Install Rust](https://www.rust-lang.org/tools/install)
1. [Install SQLite3](https://www.sqlite.org/download.html)
1. Clone or fork this repo and `cd` to it
1. Use Rust nightly version `rustup override set nightly`
1. [Install Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) `cargo install diesel_cli --no-default-features --features sqlite`
1. Setup database `diesel setup`
1. And then `cargo run`

### Code coverage

To evaluate code coverage, use [Tarpaulin](https://github.com/xd009642/tarpaulin).

1. [Install Docker](https://docs.docker.com/get-docker/)
1. Use Docker image of Tarpaulin `docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin:develop-nightly sh -c "apt-get update && apt-get install --no-install-recommends -y sqlite3 libsqlite3-dev && cargo tarpaulin -v --exclude-files dice-roller/*"`
