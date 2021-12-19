ARG RUST_TARGET
FROM rustembedded/cross:${RUST_TARGET}-0.2.1

ARG DEB_ARCH
ARG GCC_TARGET

RUN dpkg --add-architecture $DEB_ARCH
RUN apt-get update
RUN apt-get install --assume-yes libsystemd-dev:$DEB_ARCH libdbus-1-dev:$DEB_ARCH libdbus-1-dev:$DEB_ARCH liblzma-dev:$DEB_ARCH liblz4-dev:$DEB_ARCH libgcrypt-dev:$DEB_ARCH libgpg-error-dev:$DEB_ARCH pkg-config curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup-init
RUN chmod +x /usr/local/bin/rustup-init
RUN /usr/local/bin/rustup-init -y
ENV PATH "/root/.cargo/bin:$PATH"
RUN rustup toolchain add 1.56.1 --profile minimal

env PKG_CONFIG_ALLOW_CROSS "1"
env PKG_CONFIG_LIBDIR "/usr/lib/$GCC_TARGET/pkgconfig"
env PKG_CONFIG_SYSROOT_DIR "/usr/$GCC_TARGET"
env PKG_CONFIG_SYSTEM_LIBRARY_PATH "/usr/lib/$GCC_TARGET"
env PKG_CONFIG_SYSTEM_INCLUDE_PATH "/usr/$GCC_TARGET/include"
