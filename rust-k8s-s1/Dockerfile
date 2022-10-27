FROM ubuntu:jammy
LABEL Ubuntu Run Test Rust Jammy 1

COPY build/binary/http_server_s1 /app/http_server_s1

WORKDIR /app

EXPOSE 3100
EXPOSE 3101

CMD ["/app/http_server_s1"]
