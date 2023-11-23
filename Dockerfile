FROM rust:1.73.0 as builder

RUN apt-get update \
  && apt-get install -y protobuf-compiler \
  && apt-get install -y openssl \
  && rm -rf /var/lib/apt/lists/*

ADD . /app/
WORKDIR /app

COPY . .
RUN cargo build --release
RUN rm ./target/release/deps/colatiger*
RUN cp ./target/release/colatiger /app

FROM debian:latest
RUN apt-get update \
  && apt-get install -y openssl \
  && rm -rf /var/lib/apt/lists/*

#
COPY --from=builder /app/colatiger /
COPY --from=builder /app/config /config

ENV RUST_LOG="colatiger=debug,tower_http=debug,info"
ENV mode=dev

EXPOSE 8080
ENTRYPOINT ["sh","-c","./colatiger"]

