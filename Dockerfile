FROM rust:latest AS build

WORKDIR /usr/src/al-verification-bot
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim AS install

WORKDIR /app
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/al-verification-bot /usr/local/bin/al-verification-bot
CMD ["al-verification-bot"]
