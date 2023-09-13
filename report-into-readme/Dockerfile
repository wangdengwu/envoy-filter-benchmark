FROM rust:1.71.1 as builder
WORKDIR /usr/src/report-into-readme
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/report-into-readme /usr/local/bin/report-into-readme
CMD ["report-into-readme"]