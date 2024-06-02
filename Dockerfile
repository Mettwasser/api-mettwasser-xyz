FROM rust:latest

COPY . .

# Download TailwindCSS CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.0/tailwindcss-linux-x64
RUN chmod +x tailwindcss-linux-x64
RUN mv tailwindcss-linux-x64 tailwindcss

# Build TailwindCSS
RUN ./tailwindcss -c ./tailwind.config.js -i ./assets/styles/index.css -o ./build/index.css --minify

# Build Rust
RUN cargo build --release

# Delete tailwind
RUN rm -f tailwindcss \
    rm -f tailwind.config.js

# Run the app
ENTRYPOINT ./target/release/api-mettwasser-xyz
