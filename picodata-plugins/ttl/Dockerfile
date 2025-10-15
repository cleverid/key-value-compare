FROM rust:1.90-bullseye

RUN cargo install picodata-pike

WORKDIR /app
COPY . .

CMD ["cargo", "pike", "plugin", "pack"]
