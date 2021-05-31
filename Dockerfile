FROM rust:1.52-alpine as builder
WORKDIR /build
RUN apk add alpine-sdk
COPY . .
RUN cargo build --release

FROM alpine:3
WORKDIR /fn
COPY --from=builder  /build/target/release/simple-calc .
CMD ["./simple-calc"]
