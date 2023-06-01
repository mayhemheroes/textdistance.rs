FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /textdistance
WORKDIR /textdistance/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /textdistance/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_textdistance /