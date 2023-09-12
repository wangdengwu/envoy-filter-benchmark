FROM rust:1.71.1 as builder
WORKDIR /usr/src/report-into-readme
COPY . .
RUN cargo install --path .
CMD ["report-into-readme"]