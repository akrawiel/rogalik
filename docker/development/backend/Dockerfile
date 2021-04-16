FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /app

EXPOSE 9000

VOLUME ["/usr/local/cargo"]
