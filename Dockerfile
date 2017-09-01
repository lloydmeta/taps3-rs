FROM ekidd/rust-musl-builder as builder

USER rust

COPY . /home/rust/src

# Need to have the rust user own our copied source files
RUN sudo chown -R rust . &&\
    cargo build --release --verbose

FROM alpine

ARG VCS_REF
ARG CA_CERT
ARG BUILD_DATE

LABEL org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/lloydmeta/taps3-rs" \
      org.label-schema.build-date=$BUILD_DATE

COPY $CA_CERT /etc/ssl/certs/
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/taps3 /taps3
COPY entry.sh /entry.sh

RUN addgroup -S taps3user &&\
    adduser -S -g taps3user taps3user &&\
    chown -R taps3user /etc/ssl/certs/ &&\
    chown taps3user /taps3 &&\
    chown taps3user /entry.sh

USER taps3user

ENTRYPOINT ["/entry.sh"]