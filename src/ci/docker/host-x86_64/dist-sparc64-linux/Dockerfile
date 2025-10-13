FROM ubuntu:22.04

COPY scripts/cross-apt-packages.sh /scripts/
RUN sh /scripts/cross-apt-packages.sh

COPY scripts/crosstool-ng.sh /scripts/
RUN sh /scripts/crosstool-ng.sh

COPY scripts/rustbuild-setup.sh /scripts/
RUN sh /scripts/rustbuild-setup.sh
WORKDIR /tmp

COPY scripts/crosstool-ng-build.sh /scripts/
COPY host-x86_64/dist-sparc64-linux/sparc64-linux-gnu.defconfig /tmp/crosstool.defconfig
RUN /scripts/crosstool-ng-build.sh

ENV PATH=$PATH:/x-tools/sparc64-unknown-linux-gnu/bin

# openssl-src-rs doesn't know what sparc64-linux is.
# Once https://github.com/alexcrichton/openssl-src-rs/pull/271 is merged, released, and openssl-sys is updated,
# we can stop building OpenSSL the hard way.
COPY scripts/openssl.sh /scripts/
RUN /scripts/openssl.sh openssl-3.5.0 linux64-sparcv9 --cross-compile-prefix=/x-tools/sparc64-unknown-linux-gnu/bin/sparc64-unknown-linux-gnu-
ENV SPARC64_UNKNOWN_LINUX_GNU_OPENSSL_NO_VENDOR=y
ENV SPARC64_UNKNOWN_LINUX_GNU_OPENSSL_DIR="/openssl"

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV \
    CC_sparc64_unknown_linux_gnu=sparc64-unknown-linux-gnu-gcc \
    AR_sparc64_unknown_linux_gnu=sparc64-unknown-linux-gnu-ar \
    CXX_sparc64_unknown_linux_gnu=sparc64-unknown-linux-gnu-g++

ENV HOSTS=sparc64-unknown-linux-gnu

ENV NO_DOWNLOAD_CI_LLVM=1

ENV RUST_CONFIGURE_ARGS --enable-extended --enable-profiler --disable-docs
ENV SCRIPT python3 ../x.py dist --host $HOSTS --target $HOSTS
