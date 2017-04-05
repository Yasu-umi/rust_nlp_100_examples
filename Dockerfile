FROM ubuntu:16.04
MAINTAINER "Yasuumi Nishikawa" <yasu.umi.19910101@gmail.com>

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get upgrade -qy && \
    apt-get install -y --no-install-recommends \
    curl git openssl libssl-dev ca-certificates \
    build-essential pkg-config autoconf libtool \
    mecab mecab-ipadic-utf8 libmecab-dev gnuplot5 fonts-ipafont-gothic && \
    apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/*

RUN mkdir ~/tmp && cd ~/tmp && \
    git clone https://github.com/taku910/crfpp.git && cd crfpp && \
    sed -i '/#include "winmain.h"/d' crf_test.cpp && sed -i '/#include "winmain.h"/d' crf_learn.cpp && \
    ./configure && make && make install && \
    git clone https://github.com/taku910/cabocha.git && cd cabocha && \
    LDFLAGS="-Wl,-rpath=/usr/local/lib -L/usr/local/lib" ./configure --with-charset=UTF8 --enable-utf8-only && \
    make && make install && \
    rm -rf ~/tmp

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
