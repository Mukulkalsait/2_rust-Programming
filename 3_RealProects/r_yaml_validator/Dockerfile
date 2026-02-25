
# |============== Building ===============|
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /r_yaml_verifier

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# |============== Runtime ===============|
FROM alpine:latest
WORKDIR /home/x/a
COPY text.yaml .
COPY --from=builder /r_yaml_verifier/target/x86_64-unknown-linux-musl/release/r_yaml_verifier .
ENTRYPOINT ["/home/x/a/r_yaml_verifier"]
CMD ["--", "/home/x/a/text.yaml"]
