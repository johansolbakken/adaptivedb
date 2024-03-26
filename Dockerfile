# Use the official Rust image as a base to avoid having to install Rust manually
FROM rust:latest

# Create a new empty shell project
RUN USER=root cargo new --bin adaptivedb
WORKDIR /adaptivedb

# Copy your Cargo.toml and Cargo.lock to get your dependencies
COPY ./Cargo.toml ./Cargo.lock ./

# This step caches your dependencies to not rebuild them on every build
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code into the Docker image
COPY ./src ./src

# Build your project with the release profile
RUN touch src/main.rs && cargo build --release

# Expose the port your application listens on (if applicable)
EXPOSE 3000

# Command to run your executable
CMD ["./target/release/adaptivedb"]
