FROM docker.io/rust:trixie AS builder

WORKDIR /app
ADD . /app

RUN cargo build --release

FROM docker.io/debian:trixie

WORKDIR /app
COPY --from=builder /app/target/release/monday /app

CMD ["/app/monday"]
