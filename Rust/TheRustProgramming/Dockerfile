FROM rust:latest

#1.事始
WORKDIR /usr/src/projects/

#1.2 Hello,World!
#RUN mkdir /usr/src/projects/hello_world
COPY ./hello_world ./hello_world
RUN cd /usr/src/projects/hello_world && \
    rustc main.rs
#CMD ["./main"]

#1.3 Hello,Cargo
ENV USER=root
RUN cargo new hello_cargo --bin && \
    cd /usr/src/projects/hello_cargo && \
    cargo build
#CMD ["/usr/src/projects/hello_cargo/target/debug/hello_cargo"]

# 2.数当てゲームをプログラミングする
RUN cargo new guessing_game --bin
COPY ./guessing_game ./guessing_game
RUN cd /usr/src/projects/guessing_game && \
    cargo build
WORKDIR /usr/src/projects/guessing_game
CMD ["cargo", "run"]
