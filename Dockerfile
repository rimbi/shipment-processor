FROM rust

WORKDIR /app
COPY . .

CMD ["/bin/bash"]
