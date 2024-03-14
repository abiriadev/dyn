FROM rust

WORKDIR /dyn

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./tree-sitter-dyn ./tree-sitter-dyn
COPY ./crates ./crates

RUN ["cargo", "install", "--path", "."]

ENTRYPOINT ["dyn"]
