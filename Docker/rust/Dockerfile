FROM rust:1.55

WORKDIR /fun-location-backend/flat_backend
# WORKDIR /app
# WORKDIR /fun-location-backend/flat_backend
# COPY . .
# COPY /fun-location-backend/flat_backend/Cargo.toml Cargo.toml

RUN apt-get update
RUN apt-get -y install default-mysql-client
# COPY /Docker/rust/wait-for-it.sh .

# RUN ./wait-for-it.sh mysql:3306 -- mysql -udocker -p9Xu84SA1 -h mysql diesel_demo

RUN cargo install diesel_cli --no-default-features --features mysql 
# RUN cargo install --path ./src

COPY /flat_backend .
COPY /scripts .
# RUN diesel setup
# RUN diesel migration run 