FROM rustlang/rust:nightly
WORKDIR /app
COPY . .

ENTRYPOINT [ "./app" ]

RUN cargo build --release

CMD cargo run --release