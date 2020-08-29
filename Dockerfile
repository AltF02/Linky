FROM rustlang/rust:nightly

WORKDIR /usr/src/app
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build

EXPOSE 3000

CMD ["linky"]