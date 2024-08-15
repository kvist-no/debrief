FROM rust:1.79.0 as builder
WORKDIR /usr/src/debrief
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl ca-certificates

COPY --from=builder /usr/local/cargo/bin/debrief /usr/local/bin/debrief
CMD ["debrief"]