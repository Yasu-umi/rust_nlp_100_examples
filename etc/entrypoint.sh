#!/bin/bash

cp etc/nginx.conf /etc/nginx/nginx.conf && \
service nginx restart

redis-server /etc/redis/redis.conf

mkdir -p /root/rust_nlp_100_examples/tmp/mongo
mkdir -p /root/rust_nlp_100_examples/tmp/log
mongod --fork \
    --dbpath /root/rust_nlp_100_examples/tmp/mongo \
    --logpath /root/rust_nlp_100_examples/tmp/log/mongodb.log \
    --bind_ip 0.0.0.0