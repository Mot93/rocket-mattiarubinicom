FROM docker.io/rust:1.55-alpine3.13 as build_mattiarubinicom

WORKDIR /usr/src/myappcode

COPY homepage/Cargo.toml Cargo.toml
COPY homepage/src/ src/

RUN apk add libc-dev \
&& cargo build --release

FROM docker.io/alpine

WORKDIR /usr/src/myapp

COPY homepage/Rocket.toml Rocket.toml
COPY homepage/static/ static/
COPY homepage/templates/ templates/
COPY --from=build_mattiarubinicom /usr/src/myappcode/target/release/mattiarubinicom mattiarubinicom

RUN chmod u+x mattiarubinicom

CMD ["/usr/src/myapp/mattiarubinicom"]