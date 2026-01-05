FROM rust:alpine3.23 AS build
COPY . /src
WORKDIR /src
RUN ./build.sh

FROM alpine:3.23
COPY --from=build /src/target/release/tuggy /usr/bin/tuggy
ENTRYPOINT ["/usr/bin/tuggy"]
