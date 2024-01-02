# build tailwind
FROM d3fk/tailwindcss:latest as tw
COPY ./ ./

CMD [ "./tailwindcss", "-c", "./tailwind.config.js", "-i", "./assets/index.css", "-o", "./build/index.css" ]


# Build rust and run it
FROM rust:1.75
COPY ./ ./

RUN cargo build --release
CMD [ "./target/release/api-mettwasser-xyz" ]