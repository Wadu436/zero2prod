watch:
    cargo watch -x check -x test -x run

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