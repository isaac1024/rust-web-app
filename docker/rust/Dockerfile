FROM rust:1-alpine

WORKDIR /app

RUN apk update && apk add --no-cache musl-dev openssl-dev
RUN cargo install cargo-watch

COPY ./init /usr/local/bin/
RUN chmod +x /usr/local/bin/init

CMD ["init"]
