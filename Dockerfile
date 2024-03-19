# Use an official base image that includes the necessary build tools (CMake, Ninja, g++, etc.)
FROM ubuntu:latest

# Install build dependencies (you may need to update or extend this list based on your project's dependencies)
RUN apt-get update && apt-get install -y \
    cmake \
    ninja-build \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Copy your project files into the Docker image
COPY . /AdaptiveDB

# Set the working directory to your project's directory
WORKDIR /AdaptiveDB

# Run CMake with Ninja to build your project
RUN cmake -B build -G Ninja \
    && cmake --build build

# Expose the port your application listens on (if applicable)
EXPOSE 3000

# Command to run your executable
CMD ["./build/AdaptiveDB"]
