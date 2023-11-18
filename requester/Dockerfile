FROM rust:1-alpine AS chef
# Use apk for package management in Alpine
RUN apk add --no-cache build-base libressl-dev
WORKDIR /app
COPY . .
ENTRYPOINT [ "cargo", "run", "--release" ]
