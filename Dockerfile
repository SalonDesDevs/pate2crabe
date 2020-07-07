FROM ubuntu:latest

RUN apt update && \
    DEBIAN_FRONTEND=noninteractive apt install -yq \ 
    curl \
    libudev-dev \
    alsa \ 
    build-essential \
    pkg-config \
    libasound2-dev
    
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /app

ENTRYPOINT /root/.cargo/bin/rustup install . && /root/.cargo/bin/cargo build --release --target-dir=/target