# Use a Rust image as the base image
FROM rust:1.67 as build

# Set the working directory inside the container
WORKDIR /usr/src/sidecar

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies of the Rust application
RUN cargo build --release

# Copy the rest of the application source code to the container
COPY . . 

RUN cargo build --release

# Use a minimal image as the runtime image
FROM debian:buster-slim

# Install any runtime dependencies needed by the application
RUN apt-get update && apt-get install -y git

# Set the working directory inside the runtime container
WORKDIR /usr/src/sidecar

# Copy the compiled application and its dependencies from the builder container into the runtime container
COPY --from=builder /usr/src/myapp/target/release/sidear /usr/src/sidecar

# Set the default command to run the Rust application
CMD ["./sidecar"]
