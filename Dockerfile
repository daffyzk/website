FROM rust as gaming

WORKDIR /work

COPY . .

RUN cargo build --release

CMD ["target/release/daffy-website"]

# FROM scratch
# COPY --from=gaming /work/target/release /

# ENTRYPOINT ["/daffy-website"]
# # CMD ["ls"]
