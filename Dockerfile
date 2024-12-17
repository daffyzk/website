FROM rust as gaming

WORKDIR /work

COPY . .

# need to build it with 'bins' to have all the 
RUN cargo build --bins --release

CMD ["/target/release/daffy-website"]

# FROM scratch
# COPY --from=gaming /work/target/release /

# ENTRYPOINT ["/daffy-website"]
# # CMD ["ls"]