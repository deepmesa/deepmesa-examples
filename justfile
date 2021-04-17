
all: build test

build:
    cargo build --all

test:
    cargo test --all -- --nocapture

doc:
    cargo doc --no-deps --all

clean:
    cargo clean

release-minor:
    cargo release minor --workspace
