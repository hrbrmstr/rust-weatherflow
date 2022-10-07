binary := "weatherflow"

build:
	cargo build --release

run:
	cargo run --release

build-and-sign:
  cargo build --target=aarch64-apple-darwin --release && \
  cargo build --target=x86_64-apple-darwin --release && \
  lipo -create -output "${HOME}/bin/{{binary}}" "target/aarch64-apple-darwin/release/{{binary}}" "target/x86_64-apple-darwin/release/{{binary}}" && \
  codesign --force --verify --verbose --sign "${APPLE_SIGN}" "${HOME}/bin/{{binary}}"