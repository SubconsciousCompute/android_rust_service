#!/bin/bash
# Bootstrap a OpenSUSE box

set -e

# setup rust 
if ! command -v rustup &> /dev/null; then
    echo "rustup not found. installing...."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

export PATH=$HOME/.cargo/bin:$PATH
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk

export ANDROID_HOME=$HOME/Android
ANDROID_NDK=$ANDROID_HOME/android-ndk-r25c

if [ ! -d "$ANDROID_NDK" ]; then
    echo "Installing android ndk "
    wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip -O temp.zip
    unzip -d "$ANDROID_HOME" temp.zip
    rm temp.zip
fi

if [ -f "$ANDROID_NDK/ndk-build" ]; then
    echo "ndk-build found in $ANDROID_NDK. writing ndk.dir to local.properties"
else
    echo Could not locate binary ndk-build. 
    exit 1
fi

# accept license.
mkdir "$ANDROID_HOME/licenses"
echo "24333f8a63b6825ea9c5514f83c2829b004d1fee" > "$ANDROID_HOME/licenses/android-sdk-license"

# write local.properties
LOCAL_PROPERTIES_FILE="local.properties"
echo "ndk.dir=$ANDROID_NDK" > "$LOCAL_PROPERTIES_FILE"

# install stupid java17
# https://www.oracle.com/java/technologies/downloads/#java17
export JAVA_HOME=/usr/lib/jvm/jdk-17-oracle-x64
if [ ! -f $JAVA_HOME/bin/java ]; then
    echo "Installing stupid java-17 from Oracle link..."
    sudo zypper -n --no-gpg-checks install https://download.oracle.com/java/17/latest/jdk-17_linux-x64_bin.rpm
fi

echo [OK] Bootstraping done
