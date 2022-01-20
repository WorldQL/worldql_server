FROM ubuntu:20.04 as build
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update

RUN apt-get install --no-install-recommends -y \
    build-essential cmake \
    ca-certificates curl

ENV RUSTUP_HOME=/rust
ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:/rust/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN mkdir /server
COPY . /server
WORKDIR /server
RUN cargo build --release
RUN strip target/release/worldql_server

FROM ubuntu:20.04
COPY --from=build /server/target/release/worldql_server /usr/local/bin

CMD ["/usr/local/bin/worldql_server"]
ENTRYPOINT ["/usr/local/bin/worldql_server"]
