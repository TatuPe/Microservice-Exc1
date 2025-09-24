FROM rust:trixie

WORKDIR /usr/src/myapp

COPY . .

# Line below was stated in the official instructions
# but "cargo build" works better.
#RUN cargo install --path .

RUN cargo build

CMD [ "cargo", "run" ]