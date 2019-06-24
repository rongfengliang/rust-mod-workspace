FROM rustlang/rust:nightly AS build

WORKDIR /app

COPY . /app

RUN cargo clean && cargo build --release --all

FROM debian:stretch-slim

WORKDIR /app

COPY --from=build /app/target/release/mod-app /usr/local/bin/mod-app
COPY --from=build /app/target/release/usersmain /usr/local/bin/usersmain

CMD [ "usersmain" ]