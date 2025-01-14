FROM rust as gaming

WORKDIR /work

COPY . .

# RUN rustup target add x86_64-unknown-linux-musl && \
#	cargo build --release --target x86_64-unknown-linux-musl

# for the raspi cluster
RUN rustup target add aarch64-unknown-linux-musl && \
    cargo build --release --target aarch64-unknown-linux-musl

FROM scratch
#COPY --from=gaming /work/target/x86_64-unknown-linux-musl/release/daffy-website /

COPY --from=gaming /work/target/aarch64-unknown-linux-musl/release/daffy-website /

ENTRYPOINT ["/daffy-website"]
