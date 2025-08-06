FROM rust:alpine
WORKDIR /build
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo b --release
ENTRYPOINT ["./target/release/open_housing"]



