FROM rust:1.43-buster

RUN apt update &&\ 
    apt upgrade -y

RUN rustup update nightly &&\ 
    rustup default nightly

WORKDIR /usr/src/myappcode

COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY templates/ templates/

RUN cargo build --release

FROM debian:buster-slim

RUN apt update &&\ 
    apt upgrade -y

WORKDIR /usr/src/myapp

COPY Rocket.toml Rocket.toml
COPY static/ static/
COPY templates/ templates/
COPY --from=0 /usr/src/myappcode/target/release/mattiarubinicom mattiarubinicom

RUN chmod 777 mattiarubinicom

CMD ["/usr/src/myapp/mattiarubinicom"]