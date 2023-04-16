FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /steel
WORKDIR /steel/fuzz
RUN cargo +nightly fuzz build 

FROM ghcr.io/evanrichter/cargo-fuzz
COPY --from=builder /steel/fuzz/target/x86_64-unknown-linux-gnu/release/steel-fuzz /
# RUN apt update && apt install libssl-dev -y