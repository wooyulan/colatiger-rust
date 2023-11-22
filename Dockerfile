FROM rust:1.73.0 as builder

RUN USER=root cargo new --bin colatiger
WORKDIR /colatiger
COPY ./Cargo.toml ./Cargo.toml
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

# Build web app with own code
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/colatiger*
RUN cargo build --release


FROM debian:latest

ARG APP=/usr/src/app


RUN apt-get update \
    && apt install -y protobuf-compiler \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* 

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /colatiger/target/release/colatiger ${APP}/colatiger
COPY --from=builder /colatiger/config ${APP}/config

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV mode=dev
CMD ["env=$mode ./colatiger"]