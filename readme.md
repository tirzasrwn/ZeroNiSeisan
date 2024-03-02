# zero2seisan

## About

Learning rust for production ready. zero2seisan should be read in Japanese, zero ni seisan (０ に 生産), means zero to production.

## Requirement

- cargo-watch
- cargo-tarpaulin
- clippy
- rustfmt
- cargo-audit
- clang
- lld
- sqlx-cli
- postgresql-client-14
- postgresql-client-common
- cargo-undeps
- bunyan

## Command

```sh
# Install nightly compiler.
rustup toolchain install nightly --allow-downgrade
# Linux watch like. Run cargo check, test, and run on save.
cargo watch -x check -x test -x run
# Enable trace log.
RUST_LOG=trace cargo run
# Pipe the output to bunyan for more readeble output.
RUST_LOG=trace cargo run | bunyan
# Build docker image.
docker build --tag zero2seisan --file Dockerfile .
# Generate .sqlx directory for offline mode sqlx.
cargo sqlx prepare --workspace
```

## Database migration

```sh
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
sqlx migrate add create_subscriptions_table
sqlx migrate run
```
