FROM rust:1.92-trixie AS build
COPY . /src
WORKDIR /src
RUN cargo install --path .

FROM debian:trixie
COPY --from=build /src/target/release/tuggy /usr/bin
ENTRYPOINT ["/usr/bin/tuggy"]
