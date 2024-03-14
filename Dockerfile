FROM rust

WORKDIR /dyn

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./crates ./crates

RUN ["cargo", "build", "--release", "--package", "dyn-cli"]

ENTRYPOINT ["./target/release/dyn"]
