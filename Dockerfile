FROM rust:1.50 AS build

WORKDIR /usr/src
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim AS runtime
COPY --from=build /usr/src/target/release/relaxdays-hackathon-cc-vol1-7-fubini-numbers ./
ENTRYPOINT ["./relaxdays-hackathon-cc-vol1-7-fubini-numbers"]
