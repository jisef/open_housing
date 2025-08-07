FROM rust:alpine
WORKDIR /build
RUN apk add --no-cache musl-dev # only on aarch
COPY . .
RUN cargo b --release
ENTRYPOINT ["./target/release/open_housing"]



