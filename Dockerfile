FROM rust as gaming

WORKDIR /work

COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=gaming /work/target/x86_64-unknown-linux-musl/release/daffy-website /

ENTRYPOINT ["/daffy-website"]
