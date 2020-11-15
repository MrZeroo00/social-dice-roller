FROM liuchong/rustup:stable

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    sqlite3 libsqlite3-dev

ADD . /app
WORKDIR /app
RUN rustup override set nightly
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel setup
RUN cargo build
CMD ["cargo", "run"]
