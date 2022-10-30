
FROM rust:1.54.0

WORKDIR /app
RUN apt update &&\
    rm -rf ~/.cache &&\
    apt clean all &&\
    apt install -y cmake &&\
    apt install -y clang