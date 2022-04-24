FROM rustlang/rust:nightly

ARG PORT
ARG DB_INFO

ENV HOST_ADDRESS=0.0.0.0
ENV ENV=dev

WORKDIR /app
COPY . .

RUN cargo build --release

CMD WARP_PORT=$PORT HOST_ADDRESS=$HOST_ADDRESS DB_INFO=$DB_INFO ./target/release/rust-warp-bb8-postgres