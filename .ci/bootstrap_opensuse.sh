#!/bin/bash

# Bootstrap the environment on OpenSUSE.

set -e

ANDROID_HOME=$HOME/Android
ANDROID_NDK=$ANDROID_HOME/ndk
mkdir -p $ANDROID_HOME

if [ ! -d $ANDROID_NDK ]; then
    echo "Installing android ndk "
    mkdir -p $ANDROID_NDK
    wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip -O temp.zip
    unzip -d $ANDROID_NDK temp.zip
    rm temp.zip
else
    echo "Found $ANDROID_NDK. Assuming android ndk is installed here..."
fi

# install stupid java17
# https://www.oracle.com/java/technologies/downloads/#java17
export JAVA_HOME=/usr/lib/jvm/jdk-17-oracle-x64
if [ ! -f $JAVA_HOME/bin/java ]; then
    echo "Installing stupid java-17 from Oracle link..."
    sudo zypper -n --no-gpg-checks install https://download.oracle.com/java/17/latest/jdk-17_linux-x64_bin.rpm
fi


# setup rust 
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk

echo [OK] All done
