#!/usr/bin/env bash

set -ex

hide_output() {
  set +x
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "$@" &> /tmp/build.log
  rm /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  set -x
}

OPENSSL=$1
shift
BUILD_KIND=$1
shift

# may have been downloaded in a previous run
if [ ! -d ${OPENSSL} ]; then
  URL_ROOT=https://github.com/openssl/openssl/releases/download
  curl -sSL ${URL_ROOT}/${OPENSSL}/${OPENSSL}.tar.gz | tar xzf -
fi

mkdir -p build
cd build
../$OPENSSL/Configure ${BUILD_KIND} --prefix=/openssl "$@"
hide_output make -j$(nproc)
hide_output make install
hide_output make clean
