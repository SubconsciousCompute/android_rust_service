FROM opensuse/leap:15.5
MAINTAINER Dilawar Singh <dilawar@subcom.tech>

RUN zypper -n install wget curl zip unzip cmake gcc gcc-c++ sudo
RUN zypper -n install rustup
RUN zypper -n --no-gpg-checks install https://download.oracle.com/java/17/latest/jdk-17_linux-x64_bin.rpm
ENV JAVA_HOME /usr/lib/jvm/jdk-17-oracle-x64

WORKDIR /app

# setup cargo and dependencie
ENV PATH "$HOME/.cargo/bin:$PATH"
RUN rustup default stable
RUN rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
RUN cargo install cargo-ndk

# setup NDK
ENV ANDROID_HOME "$HOME/Android"
RUN wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip -O temp.zip \
    && unzip -d "$ANDROID_HOME" temp.zip && rm temp.zip
ENV LOCAL_PROPERTIES_FILE="/app/local.properties"
RUN echo "ndk.dir=$ANDROID_NDK" > "$LOCAL_PROPERTIES_FILE"

# accept license.
RUN mkdir "$ANDROID_HOME/licenses"
RUN echo "24333f8a63b6825ea9c5514f83c2829b004d1fee" > "$ANDROID_HOME/licenses/android-sdk-license"

COPY ./.ci/build_inside_docker.sh /root/build_inside_docker.sh
CMD ["sh", "/root/build_inside_docker.sh"]
