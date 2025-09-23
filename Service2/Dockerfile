FROM rust:trixie

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

CMD [ "cargo", "run" ]