[![CircleCI](https://circleci.com/gh/Yasu-umi/rust_nlp_100_examples/tree/master.svg?style=svg)](https://circleci.com/gh/Yasu-umi/rust_nlp_100_examples/tree/master)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

build container

```
docker build -t="rust_nlp_100_examples" .
```

cargo build test

```
docker run -it --rm -v `pwd`:/root/rust_nlp_100_examples --name rust_nlp_100_examples rust_nlp_100_examples cargo build
```