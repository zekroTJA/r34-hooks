FROM rust:alpine AS build
WORKDIR /build
COPY persistence/ persistence/
COPY r34-wrapper/ r34-wrapper/
COPY runner/ runner/
COPY scraper/ scraper/
COPY Cargo.lock Cargo.toml ./
RUN apk add musl-dev
RUN cargo build -p runner --release

FROM alpine:latest
COPY --from=build /build/target/release/runner /bin/runner
ENTRYPOINT [ "/bin/runner" ]
