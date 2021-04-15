FROM rust:1.50 as builder

WORKDIR /usr/src/sudoku

COPY . .

RUN cargo install --path .



FROM debian:buster-slim

RUN mkdir -p /app/sudoku

WORKDIR /app/sudoku

COPY --from=builder /usr/local/cargo/bin/sudoku .

RUN chmod u+x sudoku

CMD ["./sudoku"]
