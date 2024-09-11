FROM rust:1.78.0 as builder
WORKDIR /usr/src/yeetlight
COPY . .
RUN cargo install --path .


FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/yeetlight /usr/local/bin/yeetlight
EXPOSE 8080
ENTRYPOINT ["yeetlight"]
