FROM rust:latest

COPY . .

# Download TailwindCSS CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64
RUN chmod +x tailwindcss-linux-arm64
RUN mv tailwindcss-linux-arm64 tailwindcss

# Build TailwindCSS
RUN ./tailwindcss -c ./tailwind.config.js -i ./assets/styles/index.css -o ./build/index.css --minify

# Build Rust
RUN cargo build --release

# Run the app
ENTRYPOINT ./target/release/api-mettwasser-xyz
