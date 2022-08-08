# Dockerfile
FROM ekidd/rust-musl-builder:stable as builder
USER root
WORKDIR /home/rust
COPY . .
EXPOSE 3306
ENV DATABASE_URL=mysql://docker:docker@0.0.0.0:3306/kakeibo_db
RUN cargo build --release

FROM alpine:latest
WORKDIR /kakeibo
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/kakeibo . 
COPY --from=builder /home/rust/docker/mysql/sql .
EXPOSE 8080
ENTRYPOINT [ "./kakeibo" ] 