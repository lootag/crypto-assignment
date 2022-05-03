FROM rust:1.57

WORKDIR app
COPY ./lootag-cryptoassignment-e2e ./lootag-cryptoassignment-e2e
COPY ./lootag-cryptoassignment-domain ./lootag-cryptoassignment-domain
COPY ./lootag-cryptoassignment-services ./lootag-cryptoassignment-services

RUN cd lootag-cryptoassignment-e2e && cargo build

WORKDIR lootag-cryptoassignment-e2e

ENTRYPOINT ["cargo", "test"]

