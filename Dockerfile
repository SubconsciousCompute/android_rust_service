FROM opensuse/tumbleweed
MAINTAINER Dilawar Singh <dilawar@subcom.tech>

RUN zypper addrepo https://download.opensuse.org/repositories/home:Aptrug/openSUSE_Tumbleweed/home:Aptrug.repo
RUN zypper -n --gpg-auto-import-keys install wget curl zip unzip \
    cmake gcc gcc-c++ sudo python3 rustup\
    android-studio

# install JAVA
ENV JAVA_HOME /usr/lib/jvm/jdk-17-oracle-x64
RUN zypper -n --no-gpg-checks install https://download.oracle.com/java/17/latest/jdk-17_linux-x64_bin.rpm

WORKDIR /root

# setup cargo and dependencie
ENV PATH "$HOME/.cargo/bin:$PATH"
RUN rustup default stable
RUN rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
RUN cargo install cargo-ndk

# # setup NDK
# ENV ANDROID_HOME "$HOME/Android"
# RUN wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip -O temp.zip \
#     && unzip -d "$ANDROID_HOME" temp.zip && rm temp.zip
# ENV LOCAL_PROPERTIES_FILE="/root/local.properties"
# RUN echo "ndk.dir=$ANDROID_HOME/android-ndk-r25c" > "$LOCAL_PROPERTIES_FILE"

# # accept license.
# RUN mkdir "$ANDROID_HOME/licenses"
# RUN echo "24333f8a63b6825ea9c5514f83c2829b004d1fee" > "$ANDROID_HOME/licenses/android-sdk-license"

COPY ./.ci/build_inside_docker.sh /root/build_inside_docker.sh
CMD ["sh", "/root/build_inside_docker.sh"]
