FROM rust:1.70.0-slim-bookworm as builder
WORKDIR /src
RUN apt-get update &&\
		apt-get install -y libssl-dev gcc musl pkg-config
COPY ./ .
RUN rustup default nightly && rustup update
RUN cargo build --release --jobs 2 -Z sparse-registry


FROM debian:stable
COPY --from=builder /src/target/release/dudo /bin/dudo
RUN apt-get update &&\
		apt-get install openssl ca-certificates &&\
		apt-get clean
RUN chmod +x /bin/dudo
WORKDIR /workdir
ENTRYPOINT ["/bin/dudo"]
