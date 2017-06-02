FROM ubuntu:16.04
MAINTAINER "Yasuumi Nishikawa" <yasu.umi.19910101@gmail.com>

ENV DEBIAN_FRONTEND noninteractive

WORKDIR /root/rust_nlp_100_examples

ENV PATH=/root/.cargo/bin:$PATH

RUN apt-get update && apt-get upgrade -qy && \
    apt-get install -y --no-install-recommends \
    curl git openssl libssl-dev ca-certificates \
    build-essential pkg-config autoconf libtool gettext \
    mecab mecab-ipadic-utf8 libmecab-dev \
    gnuplot5 fonts-ipafont-gothic graphviz \
    redis-server && \
    apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/* && \

    mkdir ~/tmp && \
    cd ~/tmp && git clone https://github.com/taku910/crfpp.git && cd ./crfpp && \
    sed -i '/#include "winmain.h"/d' crf_test.cpp && \
    sed -i '/#include "winmain.h"/d' crf_learn.cpp && \
    ./configure && make && make install && ldconfig && \
    git clone https://github.com/taku910/cabocha.git && cd ./cabocha && \
    ./configure --with-charset=UTF8 --enable-utf8-only --with-posset=IPA --with-mecab-config=`which mecab-config` && \
    make && make install && ldconfig && \
    cd ~/ && rm -r ~/tmp && \

    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y && \
    /bin/bash -c "source ~/.profile && \
    rustup update && \
    rustup default nightly"

ADD src src
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock

EXPOSE 6379

CMD ["/bin/bash"]
