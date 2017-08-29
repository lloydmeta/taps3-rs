FROM ekidd/rust-musl-builder as builder

USER rust

COPY . /home/rust/src

# Need to have the rust user own our copied source files
RUN sudo chown -R rust . \
    && cargo build --release --verbose

FROM scratch

ARG VCS_REF
ARG CA_CERT

LABEL org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://travis-ci.org/lloydmeta/taps3-rs"

COPY $CA_CERT /etc/ssl/certs/
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/taps3 /taps3

ENTRYPOINT ["/taps3"]