FROM liuchong/rustup:stable

WORKDIR /app

COPY Cargo.toml .

COPY Cargo.lock .

RUN cargo install cargo-watch

COPY . .

EXPOSE 8000

CMD [ "cargo", "watch", "-x", "run"]