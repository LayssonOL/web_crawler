# Author: Laysson O. Luz
# Date: 23/02/2024
# Description: Dockerfile for development environment of web_crawler image

# Base image with cargo 1.70+
FROM rust:alpine3.19

# Define working directory
WORKDIR /app

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Copy all files from host directory to the image one
COPY . .

