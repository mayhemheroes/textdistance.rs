# Credit to nottirb for the base-builder image and workflow
# base-builder image: https://github.com/nottirb/base-builders
# workflow example used: https://github.com/nottirb/ciborium/blob/main/mayhem/Dockerfile

# Set the project name
ARG project=textdistance.rs

# Use the base builder image for Rust projects as the builder stage
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

# Add source to the build stage
ADD . /${project}
WORKDIR /${project}

# Build the fuzz targets
RUN cd ./fuzz && cargo +nightly hfuzz build

# Use Ubuntu 20.04 as the final image
FROM ubuntu:20.04

# Copy the fuzz targets from the builder stage
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_str /