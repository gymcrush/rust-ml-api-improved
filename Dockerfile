
FROM rust:1.54.0

WORKDIR /app
RUN apt update &&\
    rm -rf ~/.cache &&\
    apt clean all &&\
    apt install -y cmake &&\
    apt install -y clang

# install libtorch=1.9.0
# https://pytorch.org/get-started/locally/
WORKDIR /