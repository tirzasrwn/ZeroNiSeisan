# zero2seisan

## About

Learning rust for production ready.

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

## Command

```sh
# cargo check, test, and run on save.
cargo watch -x check -x test -x run
```

## Database migration

```sh
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
sqlx migrate add create_subscriptions_table
sqlx migrate run
```
