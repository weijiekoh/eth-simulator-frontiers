# StageX-based Rust Builder for eth_transfer_enclave
FROM stagex/bash:sx2024.03.0@sha256:d1cbbb56847e6b1e7b879214aa6926b6fdfa210e9b42a2f612a6aea850ddeefc AS bash
FROM stagex/binutils:sx2024.03.0@sha256:3af41227e1fe6a8f9b3df9916ef4876840f33eaa172168e1db1d8f457ba011d5 AS binutils
FROM stagex/ca-certificates:sx2024.03.0@sha256:6746d2d203be3455bfc5ffd5a051c8edb73ecfd7be77c3da5a2973003a30794f AS ca-certificates
FROM stagex/coreutils:sx2024.03.0@sha256:cf4032ca6b5f912a8b9d572d527d388401b68a0c9224cc086173e46bc4e1eabe AS coreutils
FROM stagex/findutils:sx2024.03.0@sha256:475ea3488840297454f0f20b58e1b8292bf9b3944f901e3fce432fa4afeaa4cd AS findutils
FROM stagex/git:sx2024.03.0@sha256:2c11f2daf9b8c1738cbd966b6de5dd0bcfaf81b675c2d268d30f972ddab9d9df AS git
FROM stagex/musl:sx2024.03.0@sha256:7db05e6817058a512a66ea82f3b99163069424c281363c2e9a48091d0d1d3bd9 AS musl
FROM stagex/openssl:sx2024.03.0@sha256:1a2f656ced34d1ade99279c5663fcf0ec4f6526bcc50142079ef8adc080be3a9 AS openssl
FROM stagex/pkgconf:sx2024.03.0@sha256:31ce4eddaf4e777ddb51f01923089f3321ec5272ca0aa834d475f644279209b8 AS pkgconf
FROM stagex/rust:sx2024.03.0@sha256:fe22a0fcdb569cb70b8147378463fb6ff800e642be9d50542f8e25a38d90ec7f AS rust
FROM stagex/zlib:sx2024.03.0@sha256:de8f56f3ece28b14d575329bead53fc5318962ae3cb8f161a2d69710f7ec51f4 AS zlib
FROM stagex/libunwind:sx2024.03.0 AS libunwind
FROM stagex/gcc:sx2024.03.0 AS gcc
FROM stagex/llvm:sx2024.03.0 AS llvm

FROM scratch as builder
# Environment variables for Rust builds
ENV TARGET=x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CARGOFLAGS="--release --target ${TARGET}"
ENV OPENSSL_STATIC=true
ENV BINARY_NAME=eth_transfer_enclave

# Copy essential StageX components for Rust builds
COPY --from=stagex/busybox . /
COPY --from=bash . /
COPY --from=coreutils . /
COPY --from=findutils . /
COPY --from=musl . /
COPY --from=openssl . /
COPY --from=zlib . /
COPY --from=ca-certificates . /
COPY --from=binutils . /
COPY --from=pkgconf . /
COPY --from=git . /
COPY --from=rust . /
COPY --from=libunwind . /
COPY --from=gcc . /
COPY --from=llvm . /

WORKDIR /src

FROM builder as build
# Copy source code
ADD . /src

# Build the Rust binary
RUN <<-EOF
	set -eux
	echo "Building with: cargo build ${CARGOFLAGS} --bin ${BINARY_NAME}"
	cargo build ${CARGOFLAGS} --bin ${BINARY_NAME}
	
	# Find and copy the binary
	BINARY_PATH="target/${TARGET}/release/${BINARY_NAME}"
	if [ -f "${BINARY_PATH}" ]; then
		cp "${BINARY_PATH}" "/app"
		echo "Binary built successfully: ${BINARY_NAME}"
	else
		echo "Warning: Binary ${BINARY_NAME} not found at ${BINARY_PATH}"
		ls -la "target/${TARGET}/release/"
		exit 1
	fi
EOF

FROM scratch as final
COPY --from=build /app /app
ENTRYPOINT ["/app"]