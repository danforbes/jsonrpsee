FROM rust:latest AS builder
RUN rustup install nightly && rustup default nightly
RUN cargo install cargo-fuzz
WORKDIR /jsonrpsee
COPY . .
RUN cargo fuzz build --fuzz-dir fuzz
FROM ubuntu
RUN apt-get update && apt-get install -y libc6-dbg
COPY --from=builder /jsonrpsee/target/release/fuzz_module /fuzz_module
COPY ./fuzz/corpus/fuzz_module /testsuite
CMD ["/fuzz_module"]
