FROM rust:1.55

WORKDIR /fun-location-backend/flat_backend
# WORKDIR /app
# WORKDIR /fun-location-backend/flat_backend
COPY ./flat_backend .
# COPY /fun-location-backend/flat_backend/Cargo.toml Cargo.toml

RUN cargo install diesel_cli --no-default-features --features mysql

# RUN cargo install --path ./src
RUN diesel setup
RUN diesel migration run 