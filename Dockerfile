FROM rustlang/rust:nightly as build
WORKDIR /work
# Gymnastics to get cached deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p ./src \
  && echo "fn main() {}" > ./src/main.rs \
  && cargo build --release \
  && cargo clean --release --package=tontsabot \
  && rm -rf ./src
COPY . ./
RUN cargo build --release

FROM debian:stretch-slim as runtime
LABEL maintainer="Ilkka Poutanen <ilkka@ilkka.dev>"
WORKDIR /app
COPY --from=build /work/target/release/tontsabot ./
CMD ["/app/tontsabot"]
