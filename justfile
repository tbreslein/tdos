clean:
  cargo clean

format:
  rustfmt ./**/*.rs

clippy:
  cargo clippy -- -A clippy::needless_return -A clippy::op_ref -A clippy::too_many_arguments

doc:
  cargo doc --document-private-items

build: format clippy doc
  cargo build -r

test: build
  cargo nextest run -E 'binary(corries)'
  cargo test --doc
  cargo nextest run -r -E 'kind(test)'

update:
  nix flake update
  cargo update
