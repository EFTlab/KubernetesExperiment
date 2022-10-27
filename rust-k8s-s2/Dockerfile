FROM ubuntu:jammy
LABEL Ubuntu Run Test Rust Jammy 2

COPY build/binary/http_server_s2 /app/http_server_s2

WORKDIR /app

EXPOSE 3200
EXPOSE 3201

CMD ["/app/http_server_s2"]
