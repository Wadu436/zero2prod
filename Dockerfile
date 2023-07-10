FROM lukemathwalker/cargo-chef:latest-rust-1.70.0-bullseye AS chef

FROM chef as planner
WORKDIR /usr/app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
WORKDIR /usr/app
COPY --from=planner /usr/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT [ "./target/release/zero2prod" ]

FROM debian:bullseye-slim as runtime
WORKDIR /usr/app
RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./zero2prod" ]