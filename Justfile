watch:
    cargo watch -x check -x test -x run

before_commit:
    cargo sqlx prepare -- --lib
    cargo test
    just lint
    just format

lint:
    cargo clippy -- -D warnings

coverage:
    cargo tarpaulin --ignore-tests

format:
    cargo fmt

format-check:
    cargo fmt --check

audit:
    cargo audit

init_db:
    ./scripts/init_db.sh