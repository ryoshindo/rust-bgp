FROM rust:1-buster

WORKDIR /rust-bgp

COPY . .

RUN rustup default nightly && cargo build

CMD ["./target/debug/rust-bgp", "64512 10.200.100.2 64513 10.200.100.3 active"]
