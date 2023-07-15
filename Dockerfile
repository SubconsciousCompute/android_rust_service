FROM debian:latest
MAINTAINER Dilawar Singh <dilawar@subcom.tech>

ENV HOME /root
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -yqq update
RUN apt-get install -yqq wget curl zip unzip
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
ENV PATH $HOME/.cargo/bin:$PATH
RUN rustup toolchain install stable
RUN rustup target add aarch64-linux-android
RUN rustup target add armv7-linux-androideabi
RUN rustup target add i686-linux-android
RUN rustup target add x86_64-linux-android

# install java
RUN wget \
    https://download.oracle.com/java/17/archive/jdk-17.0.7_linux-x64_bin.deb -O jdk.deb \ 
    && dpkg -i --force-depends jdk.deb
RUN apt install -f -y
ENV JAVA_HOME /usr/lib/jvm/jdk-17/
ENV PATH $JAVA_HOME/bin:$PATH

# install android-ndk
ENV ANDROID_HOME $HOME/Android
RUN mkdir -p $ANDROID_HOME/ndk
RUN wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip -O /tmp/temp.zip
RUN unzip -d $ANDROID_HOME/ndk /tmp/temp.zip && rm -f /tmp/temp.zip
# accept license
RUN mkdir $ANDROID_HOME/licenses
RUN echo "24333f8a63b6825ea9c5514f83c2829b004d1fee" > $ANDROID_HOME/licenses/android-sdk-license

WORKDIR /app
RUN ls -ltrh
CMD ["./gradlew", "build"]
