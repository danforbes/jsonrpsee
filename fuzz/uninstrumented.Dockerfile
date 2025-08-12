FROM rust:latest AS builder
WORKDIR /jsonrpsee
COPY . .
RUN RUSTFLAGS="-Cinstrument-coverage" cargo build -p jsonrpsee-fuzz
FROM ubuntu
RUN apt-get update && apt-get install -y libc6-dbg
COPY --from=builder /jsonrpsee/target/release/fuzz_uninstrumented /fuzz_uninstrumented
COPY ./fuzz/corpus/fuzz_module /testsuite
CMD ["/fuzz_uninstrumented", "@@"]
