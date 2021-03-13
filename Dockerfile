FROM rust:1.50 AS build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch AS runtime
COPY --from=build /usr/src/target/x86_64-unknown-linux-musl/release/relaxdays-hackathon-cc-vol1-7-fubini-numbers ./
ENTRYPOINT ["./relaxdays-hackathon-cc-vol1-7-fubini-numbers"]