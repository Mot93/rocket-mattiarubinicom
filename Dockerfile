FROM rust:1.43-alpine as build

WORKDIR /usr/src/myappcode

COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY templates/ templates/

RUN apk add libc-dev &&\
    rustup update nightly &&\ 
    rustup default nightly &&\
    apk update &&\ 
    apk upgrade &&\
    cargo build --release

FROM alpine

WORKDIR /usr/src/myapp

COPY Rocket.toml Rocket.toml
COPY static/ static/
COPY templates/ templates/
COPY --from=build /usr/src/myappcode/target/release/mattiarubinicom mattiarubinicom

RUN apk update &&\ 
    apk upgrade &&\
    chmod 777 mattiarubinicom

CMD ["/usr/src/myapp/mattiarubinicom"]