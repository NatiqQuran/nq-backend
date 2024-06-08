FROM rust:latest as builder

WORKDIR /usr/src/nq-api
COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker

COPY --from=builder /usr/src/nq-api/target/release/nq-api /usr/local/bin/nq-api

ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

ENV DATABASE_URL=postgres://postgres:1234@localhost/base
ENV SMTP_HOST=smtp.gmail.com
ENV SMTP_PORT=465
ENV SMTP_USERNAME=value
ENV SMTP_PASSWORD=password
ENV SMTP_FROM=natiq
ENV LOG_FILE_PATH=nq_errors.log

CMD ["nq-api"]
