FROM ekidd/rust-musl-builder as builder

USER rust

COPY . /home/rust/src

RUN cargo build --release --verbose

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/taps3 /taps3

CMD ["/taps3"]