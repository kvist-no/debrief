FROM rust:1.74.1 as builder
WORKDIR /usr/src/debrief
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl

COPY --from=builder /usr/local/cargo/bin/debrief /usr/local/bin/debrief
CMD ["debrief"]