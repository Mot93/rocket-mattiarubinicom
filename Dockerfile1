FROM rust:1.43-buster

RUN apt update &&\ 
    apt upgrade -y

RUN rustup update nightly &&\ 
    rustup default nightly

WORKDIR /usr/src/myappcode

COPY Cargo.toml /usr/src/myappcode/Cargo.toml
COPY Rocket.toml /usr/src/myappcode/Rocket.toml
COPY src/ /usr/src/myappcode/src/
COPY templates/ /usr/src/myappcode/templates/
COPY static/ /usr/src/myappcode/static/

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/myapp

COPY Rocket.toml Rocket.toml
COPY --from=0 /usr/src/myappcode/templates/ templates/ 
COPY --from=0 /usr/src/myappcode/static/ static/ 
COPY --from=0 /usr/src/myappcode/target/release/mattiarubinicom mattiarubinicom

CMD ["/usr/src/myappcode/target/release/mattiarubinicom"]