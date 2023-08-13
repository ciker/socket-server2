FROM rust:latest
COPY ./ ./
RUN cargo build --release
EXPOSE 8080/tcp
CMD ["./target/release/socket-server"]