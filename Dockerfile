FROM d3fk/tailwindcss:latest
COPY ./ ./

RUN ./tailwindcss

FROM rust:latest


RUN cargo build --release

RUN ./target/release/api-mettwasser-xyz