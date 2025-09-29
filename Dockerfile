FROM docker.io/rust:slim

# Install tools
RUN rustup target add wasm32-unknown-unknown

RUN apt-get update && apt-get install -y librust-openssl-dev perl
RUN cargo install --locked cargo-leptos

# Add source code
ADD . .

# Build
RUN cargo leptos build --split

# Expose the default ports
EXPOSE 3000
EXPOSE 3001

CMD ["cargo", "leptos", "watch", "--split"]