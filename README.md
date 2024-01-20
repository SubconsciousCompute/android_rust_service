[![Android CI](https://github.com/SubconsciousCompute/android_rust_service/actions/workflows/android.yml/badge.svg)](https://github.com/SubconsciousCompute/android_rust_service/actions/workflows/android.yml)
# Android Rust Service

Android App with foreground service using Rust Native Libraries. The service
does FileSystem Monitoring and makes the logs available at
http://localhost:3000/fs

## Getting Started On Linux

A `docker-compose.yml` and `Dockerfile` are available in this repository.
`docker compose up` should build the app. This is still beta.

1. Install [rustup](https://rustup.rs)
2. Install the stable toolchain
    ```shell
    rustup toolchain install stable
    ```

3. Add (required) android targets
    ```shell
    rustup target add aarch64-linux-android
    rustup target add armv7-linux-androideabi
    rustup target add i686-linux-android
    rustup target add x86_64-linux-android
    ```
4. Install [JDK 17](https://www.oracle.com/java/technologies/javase/jdk17-archive-downloads.html)
5. Add the following environment variables
    ```shell
    export JAVA_HOME=/usr
    export ANDROID_HOME=$HOME/.android_sdk
    ```
    > _Note:_ The `java` binary should be in `$JAVA_HOME/bin/`

6. Install [command-line tools](https://developer.android.com/studio). Make sure that 
    `$ANDROID_HOME/cmdline-tools/latest/bin` contains `sdkmanager` binary. Add
    to `PATH`.
    ```shell
    mkdir -p $ANDROID_HOME/cmdline-tools
    wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip -O temp.zip
    unzip -d $ANDROID_HOME/cmdline-tools temp.zip
    mv $ANDROID_HOME/cmdline-tools/cmdline-tools $ANDROID_HOME/cmdline-tools/latest 
    rm temp.zip
    export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
    sdkmanager --help # must work
    ```

7. Accept SDK license .
    ```shell
    mkdir $ANDROID_HOME/licenses
    echo "24333f8a63b6825ea9c5514f83c2829b004d1fee" > $ANDROID_HOME/licenses/android-sdk-license
    ```

8. Install dependencies using `sdkmanager`. The following packages are required to build the app. 
    ```shell
    sdkmanager "build-tools;30.0.3" "emulator" "ndk;25.2.9519653" "patcher;v4" "platform-tools" "platforms;android-33"
    ```

    | Path                 | Version      | Description                     | Location             |
    | -------------------- | ------------ | ------------------------------- | -------------------- |
    | build-tools;30.0.3   | 30.0.3       | Android SDK Build-Tools 30.0.3  | build-tools/30.0.3   |
    | emulator             | 32.1.13      | Android Emulator                | emulator             |
    | ndk;25.2.9519653     | 25.2.9519653 | NDK (Side by side) 25.2.9519653 | ndk/25.2.9519653     |
    | patcher;v4           | 1            | SDK Patch Applier v4            | patcher/v4           |
    | platform-tools       | 34.0.3       | Android SDK Platform-Tools      | platform-tools       |
    | platforms;android-33 | 2            | Android SDK Platform 33         | platforms/android-33 |

9. Clone the repository & chdir into it and build.
    ```shell
    ./gradlew build
    ```

## Demo

### FileSystem Monitoring

<img alt="FS Monitoring" src="demo/fs.gif" />
