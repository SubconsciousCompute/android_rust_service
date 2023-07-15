#!/bin/env/sh

cd /app || exit 1
echo "PWD=$PWD"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "JAVA_HOME=$JAVA_HOME"

if [ -f "$HOME/local.properties" ]; then
    cp "$HOME/local.properties" /app/
    cat local.properties
else 
    echo "[WARN] local.properties is not found in $PWD"
fi
# ls -ltR "$ANDROID_HOME"
./gradlew build
