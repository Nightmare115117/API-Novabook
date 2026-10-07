FROM ubuntu:24.04 AS build

RUN apt-get update && apt-get install -y curl ca-certificates
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

WORKDIR /app
COPY . .

ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup toolchain install stable

RUN rm -rf target && cargo build --release

FROM ubuntu:24.04

WORKDIR /app
COPY --from=build /app/target/release/api_migrado /app/api_migrado

EXPOSE 8080
CMD [./api_migrado]