run:
  rustup run nightly cargo run

clean:
  rustup run nightly cargo clean

format:
  rustup run nightly cargo fmt

clippy:
  rustup run nightly cargo clippy -- -A clippy::needless_return -A clippy::op_ref -A clippy::too_many_arguments

doc:
  rustup run nightly cargo doc --document-private-items

build: format clippy doc
  rustup run nightly cargo test

test:
  rustup run nightly cargo test

update:
  rustup run nightly cargo update
