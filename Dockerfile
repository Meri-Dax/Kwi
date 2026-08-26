FROM rust:1.93 AS build

WORKDIR /rs/src/app

# Add diesel for migration runs
RUN cargo install diesel_cli --no-default-features --features postgres

# Add cargo-watch for dev envs
RUN cargo install cargo-watch

# Import the dependencies
ADD Cargo.toml Cargo.lock ./

# Cache the dependencies
RUN mkdir src && echo "fn main () {}" > src/main.rs
RUN cargo build --release

# Now build the actual app
COPY ./src ./src
RUN cargo build --release

# Build is done, so now manage the runtime image
FROM debian:trixie-slim AS runtime

## Retrieve diesel
COPY --from=build /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Retrieve the binary from the build stage
COPY --from=build /rs/src/app/target/release/kwi /kwi

COPY diesel.toml /diesel.toml

# Download the PQL libraries
RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy Diesel migrations
COPY ./migrations /migrations

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

CMD ["/entrypoint.sh"]

# Dev environment
FROM build AS development
RUN cargo install cargo-watch

COPY entrypoint.dev.sh /entrypoint.dev.sh
RUN chmod +x /entrypoint.dev.sh

CMD ["/entrypoint.dev.sh"]
