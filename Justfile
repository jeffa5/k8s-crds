test:
    cargo run -p k8s-crds-test

generate:
    cargo run --bin k8s-crds-codegen

check:
    find . -name Cargo.toml -not -path '*/target/*' -execdir cargo check \;

clippy:
    find . -name Cargo.toml -not -path '*/target/*' -execdir pwd \; -exec cargo clippy \;

clean:
    find . -name Cargo.toml -not -path '*/target/*' -execdir cargo clean \;

doc:
    find . -name Cargo.toml -not -path '*/target/*' -execdir cargo doc \;

publish:
    find crds -name Cargo.toml -not -path '*/target/*' -execdir cargo publish \;
