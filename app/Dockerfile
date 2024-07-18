FROM rust:1.75.0 AS build

WORKDIR /opt/app

# copy over your manifests
COPY Cargo.toml Cargo.lock ./

# build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# copy the source code
COPY . ./

# build for release
RUN rm ./target/release/deps/app*
RUN cargo build --release

# the final base image
FROM ubuntu

EXPOSE 8000
WORKDIR /opt/app

# copy the build artifact from the build stage
COPY --from=build /opt/app/target/release/app ./

# run the binary
CMD ["./app"]
