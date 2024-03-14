# Build from source

First, you need to clone the repository.

```sh
git clone https://github.com/abiriadev/dyn.git
```

## Using Cargo

```sh
$ cargo build -p dyn-cli
```

To run:

```sh
$ cargo run -p dyn-cli
```

## Using Docker

```sh
$ docker build -t dyn .
```

to run:

```sh
$ docker run --rm -it dyn
```
