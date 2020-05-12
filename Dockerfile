#---------------------------------------------------------
# Cargo build stage
#---------------------------------------------------------
FROM rust:1.41.0 as cargo-build

RUN apt-get update \
  && apt-get install musl-tools -y \
  && rustup target add x86_64-unknown-linux-musl

# Build openssl
RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm &&\
  ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic &&\
  ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux &&\
  mkdir /musl && \
  wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz && \
  tar zxvf OpenSSL_1_1_1f.tar.gz && \
  cd openssl-OpenSSL_1_1_1f && \
  CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64 && \
  make depend && \
  make -j$(nproc) && \
  make install
# Set enviroment variable
ENV PKG_CONFIG_ALLOW_CROSS 1
ENV OPENSSL_STATIC true
ENV OPENSSL_DIR /musl

# Set cargo registry
RUN echo "[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = 'ustc'\n[source.ustc]\nregistry = \"git://mirrors.ustc.edu.cn/crates.io-index\"" > /usr/local/cargo/config
WORKDIR /app
RUN USER=root cargo new --bin pegasus
WORKDIR /app/pegasus
COPY Cargo.toml Cargo.lock ./
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -rf src
COPY ./src ./src
RUN RUSTFLAGS=-Clinker=musl-gcc  cargo build --release --target=x86_64-unknown-linux-musl

#----------------------------------------------------------
# Final Stage
#----------------------------------------------------------
FROM alpine:latest as final
WORKDIR /pegasus/app
COPY --from=cargo-build /app/pegasus/target/x86_64-unknown-linux-musl/release/pegasus .
COPY templates .
COPY migrations .
COPY .env .
CMD ['./pegasus']
