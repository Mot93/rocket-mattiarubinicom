FROM rust:1.55-alpine3.13 as build

WORKDIR /usr/src/myappcode

COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY Rocket.toml Rocket.toml

RUN apk add libc-dev \
&& cargo build --release

FROM alpine

WORKDIR /usr/src/myapp

COPY Rocket.toml Rocket.toml
COPY static/ static/
COPY templates/ templates/
COPY --from=build /usr/src/myappcode/target/release/mattiarubinicom mattiarubinicom

RUN chmod u+x mattiarubinicom

CMD ["/usr/src/myapp/mattiarubinicom"]