FROM rust as builder
WORKDIR /server
COPY . .
RUN cargo build --release --bin rechess-server

FROM alpine as runtime
WORKDIR /server
COPY --from=builder /server/target/release/ /usr/local/bin
ENTRYPOINT ["/usr/local/bin/server"]