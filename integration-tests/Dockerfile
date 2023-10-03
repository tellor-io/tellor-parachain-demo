FROM rust:1.68-slim as builder
RUN apt update && apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
WORKDIR /tests
COPY . .
RUN SKIP_WASM_BUILD=1 cargo build --tests --release;  \
    find ./target/release/deps/tellor* -maxdepth 1 -perm -111 -type f -exec mv {} tellor-parachain-integration-tests \;

FROM rust:slim
COPY --from=builder /tests/tellor-parachain-integration-tests .
ENTRYPOINT ["./tellor-parachain-integration-tests"]
