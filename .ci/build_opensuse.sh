#!/bin/bash
#
set -e
set -x
source .ci/bootstrap_opensuse.sh
./gradlew build --info
