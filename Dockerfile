FROM rust:latest
COPY ./ ./
RUN cargo build --release
ENV PORT=8080
EXPOSE $PORT/tcp
CMD ["./target/release/socket-server"]