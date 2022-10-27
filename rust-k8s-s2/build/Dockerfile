FROM ubuntu:jammy
LABEL Ubuntu Test Rust Jammy 2

WORKDIR /app

RUN cat /etc/lsb-release
RUN apt-get update -y
RUN apt-get dist-upgrade -y
RUN apt-get install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV RUSTFLAGS -Awarnings
ENV PATH "/root/.cargo/bin:$PATH"

VOLUME ["/app-code", "/binary"]

CMD ["/app-code/compilation.sh"]
