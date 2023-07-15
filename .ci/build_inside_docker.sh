#!/bin/env/sh

cd /app
echo "PWD=$PWD"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "JAVA_HOME=$JAVA_HOME"

if [ -f local.properties ]; then
    cat local.properties
else 
    echo "[WARN] local.properties is not found in $PWD"
fi
# ls -ltR "$ANDROID_HOME"
./gradlew build
