FROM rust:1-alpine

WORKDIR /app

RUN apk update && apk add --no-cache musl-dev
RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
