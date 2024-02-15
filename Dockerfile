FROM rust:1.76.0-slim-bookworm AS builder

WORKDIR /usr/src/refinery
COPY . .

RUN cargo build --release

FROM debian:12.5-slim
COPY --from=builder /usr/src/refinery /usr/local/bin/refinery
CMD ["refinery"]