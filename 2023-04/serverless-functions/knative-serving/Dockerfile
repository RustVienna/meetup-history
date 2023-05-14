FROM rust:1.69
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["knative-serving"]
EXPOSE 3000