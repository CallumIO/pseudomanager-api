# Build Stage
FROM rustlang/rust:latest AS chef

RUN cargo install cargo-chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin app

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]