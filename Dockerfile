FROM ekidd/rust-musl-builder as builder

USER rust

COPY . /home/rust/src

RUN cargo build --release --verbose

FROM scratch

ARG VCS_REF

LABEL org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://travis-ci.org/lloydmeta/taps3-rs"

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/taps3 /taps3

CMD ["/taps3"]