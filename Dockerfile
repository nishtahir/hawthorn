# No official musl container yet. 
# https://github.com/rust-lang-nursery/docker-rust/issues/10
FROM clux/muslrust:nightly-2018-07-16 As builder
ADD . ./
RUN cargo build --locked --target=x86_64-unknown-linux-musl --release

# Build container using tiny alpine
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /volume/target/x86_64-unknown-linux-musl/release/hawthorn \
    /usr/local/bin/
CMD /usr/local/bin/hawthorn