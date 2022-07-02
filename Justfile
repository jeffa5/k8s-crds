test:
    cargo run -p k8s-crds-test

generate:
    cargo run --bin k8s-crds-codegen

check:
    find . -name Cargo.toml -execdir cargo check \;

clippy:
    find . -name Cargo.toml -execdir pwd \; -exec cargo clippy \;

clean:
    find . -name Cargo.toml -execdir cargo clean \;
