# build environment
FROM rust:slim-buster as build
WORKDIR /app
COPY backend ./backend/
COPY search_base ./search_base/
COPY search_engine ./search_engine/
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build -p backend --release

# production environment
FROM debian:buster-slim
COPY --from=build /app/target/release/backend ./backend
EXPOSE 80
CMD ["./backend"]
