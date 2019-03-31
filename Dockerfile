FROM rust:1.32
RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev
RUN cargo install diesel_cli --force --no-default-features --features "mysql"
WORKDIR /usr/src/myapp
RUN rustup default nightly-2018-12-06
ENV ROCKET_ADDRESS=0.0.0.0
RUN cargo install cargo-watch