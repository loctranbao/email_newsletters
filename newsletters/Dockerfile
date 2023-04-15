FROM lukemathwalker/cargo-chef:latest-rust-1 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dependency tree stays the same,
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
# build our project
RUN cargo build --release --bin zero2prod



# runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*

# copy the compiled binary from the builder environmemt
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod

# we need the configuration file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]