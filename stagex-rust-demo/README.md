# StageX Rust Builder Demo

This demonstrates how to build arbitrary Rust binaries using the StageX build pattern from the QuorumOS project.

## Overview

StageX provides reproducible, minimal container images for building applications. This demo extracts the essential StageX components needed for Rust builds and creates a reusable build environment that produces statically-linked musl binaries.

## Key Features

- **Static Linking**: Produces fully static binaries with no runtime dependencies
- **Musl Target**: Uses `x86_64-unknown-linux-musl` for maximum compatibility
- **Minimal Base**: Built from scratch for minimal attack surface
- **Reproducible**: Uses pinned StageX image hashes for consistent builds
- **Flexible**: Supports any Rust project with customizable binary names

## Files Structure

```
stagex-rust-demo/
├── Containerfile          # Basic StageX Rust builder
├── Containerfile.flexible # Flexible version with build args
├── Makefile               # Build automation
├── README.md              # This file
└── example/               # Demo Rust project
    ├── Cargo.toml
    ├── Cargo.lock
    └── src/
        └── main.rs
```

## StageX Components Used

The builder includes these essential StageX images:

- `stagex/rust` - Rust toolchain and Cargo
- `stagex/musl` - musl libc for static linking
- `stagex/openssl` - OpenSSL for TLS support
- `stagex/ca-certificates` - Root certificates
- `stagex/binutils` - Build tools
- `stagex/git` - Version control (for dependencies)
- Base utilities: `busybox`, `bash`, `coreutils`, etc.

## Build Environment

The build environment matches the QuorumOS pattern:

```dockerfile
ENV TARGET=x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CARGOFLAGS="--locked --release --target ${TARGET}"
ENV OPENSSL_STATIC=true
```

## Usage

### Quick Demo

Run the complete demo:

```bash
make demo
```

This will:
1. Build the flexible Containerfile
2. Run the container
3. Extract the binary
4. Test the extracted binary

### Build Options

#### Basic Build

```bash
make build-basic
```

Uses the basic Containerfile with hardcoded settings.

#### Flexible Build

```bash
make build-flexible BINARY_NAME=my-app
```

Uses the flexible Containerfile with custom binary name.

### Custom Rust Project

To use with your own Rust project:

1. Copy `Containerfile.flexible` to your project root
2. Build with your binary name:

```bash
docker build \
  --file Containerfile.flexible \
  --build-arg BINARY_NAME=your-binary-name \
  --tag your-image:latest \
  .
```

3. Extract the binary:

```bash
docker create --name temp your-image:latest
docker cp temp:/app ./your-binary-name  
docker rm temp
```

## Binary Properties

The resulting binaries have these properties:

- **Statically linked**: No runtime dependencies
- **musl libc**: Compatible with any Linux distribution
- **Position Independent**: Built with PIE for security
- **Optimized**: Release build with full optimizations
- **Minimal size**: Stripped of debug symbols

## Verification

Test that your binary is statically linked:

```bash
file ./hello-stagex
# Output: hello-stagex: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, stripped

ldd ./hello-stagex
# Output: statically linked
```

## Customization

### Adding Dependencies

To add system dependencies, modify the Containerfile to include additional StageX images:

```dockerfile
FROM stagex/your-dependency:sx2024.03.0@sha256:... AS your-dependency
# ...
COPY --from=your-dependency . /
```

### Build Arguments

The flexible Containerfile supports these build arguments:

- `BINARY_NAME`: Name of the binary to build
- `EXTRA_CARGOFLAGS`: Additional cargo flags

### Different Targets

To build for different targets, modify the `TARGET` environment variable:

```dockerfile
ENV TARGET=aarch64-unknown-linux-musl
```

## Comparison with QOS

This demo uses the same StageX components and build pattern as QuorumOS but removes:

- AWS Nitro Enclave specific components (`eif_build`, `linux-nitro`)
- Hardware security module support (`pcsc-lite`)
- LLVM components (unless needed)
- Additional QOS-specific tools

## Security Considerations

- All StageX images use cryptographically signed hashes
- Builds are reproducible and verifiable
- Static linking eliminates runtime dependency vulnerabilities
- Minimal base reduces attack surface

## Next Steps

To use this pattern in production:

1. **Update hashes**: Use the latest StageX image hashes
2. **Add security scanning**: Integrate with vulnerability scanners
3. **Multi-arch builds**: Add support for ARM64 and other architectures
4. **CI/CD integration**: Automate builds in your pipeline
5. **Registry publishing**: Push images to your container registry

## References

- [StageX Project](https://stagex.tools)
- [QuorumOS Source](https://github.com/turnkey-hq/qos)
