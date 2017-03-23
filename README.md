build container

```
docker build -t="rust_nlp_100_examples" .
```

cargo build test

```
docker run -it --rm -v `pwd`:/root/rust_nlp_100_examples --name rust_nlp_100_examples rust_nlp_100_examples
```