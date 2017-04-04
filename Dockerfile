FROM ubuntu:16.04
MAINTAINER "Yasuumi Nishikawa" <yasu.umi.19910101@gmail.com>

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get upgrade -qy && \
    apt-get install -y --no-install-recommends \
    curl openssl libssl-dev ca-certificates build-essential pkg-config \
    mecab mecab-ipadic-utf8 libmecab-dev gnuplot5 fonts-ipafont-gothic && \
    apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y && \
    /bin/bash -c "source ~/.profile && \
    rustup update && \
    rustup default nightly"

WORKDIR /root/rust_nlp_100_examples

ADD src src
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock

ENTRYPOINT ["/bin/bash"]
CMD ["-c", "/root/.cargo/bin/cargo build"]
