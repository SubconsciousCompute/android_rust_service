FROM debian:latest
MAINTAINER Dilawar Singh <dilawar@subcom.tech>

ENV HOME /root
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -yqq update
RUN apt-get install -yqq wget curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
ENV PATH=$HOME/.cargo/bin:$PATH
RUN rustup toolchain install stable
RUN rustup target add aarch64-linux-android
RUN rustup target add armv7-linux-androideabi
RUN rustup target add i686-linux-android
RUN rustup target add x86_64-linux-android

# install java
RUN wget \
    https://download.oracle.com/java/17/archive/jdk-17.0.7_linux-x64_bin.deb -O jdk.deb \ 
    && dpkg -i --force-depends jdk.deb
RUN apt-get install -f -y

WORKDIR /app
RUN ls -ltrh
RUN ./gradlew build
