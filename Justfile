test:
    cargo run -p k8s-crds-test

generate:
    cargo run --bin k8s-crds-codegen

fmt:
    find . -name *.rs -exec rustfmt {} \;
