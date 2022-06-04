# syntax=docker/dockerfile:1
FROM ubuntu:latest
RUN apt update
RUN apt install -y curl
RUN curl https://sh.rustup.rs -sSf | sh /dev/stdin -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
RUN apt-get install -y musl-tools
RUN apt-get install -y zip
RUN apt-get install -y build-essential