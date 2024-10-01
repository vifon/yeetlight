FROM --platform=$BUILDPLATFORM rust:1.78.0 as builder
COPY --from=tonistiigi/xx / /
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN if [ "$TARGETPLATFORM" != "$BUILDPLATFORM" ]; then \
      apt-get update && \
      apt-get install -y gcc-"$(xx-info march)"-linux-gnu && \
      apt-get clean && rm -rf /var/lib/apt/lists/*; \
    fi

WORKDIR /usr/src/yeetlight
COPY . .
RUN rustup target add "$(xx-info march)"-unknown-linux-gnu
RUN cargo install --path=. --target="$(xx-info march)"-unknown-linux-gnu


FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/yeetlight /usr/local/bin/yeetlight
EXPOSE 8080
ENTRYPOINT ["yeetlight"]
