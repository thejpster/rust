# `powerpc-unknown-none-eabi`

**Tier: 3**

Rust for bare-metal 32-bit PowerPC systems, using the Embedded ABI.

| Target                    | Descriptions                              |
| ------------------------- | ----------------------------------------- |
| powerpc-unknown-none-eabi | PowerPC 32-bit (freestanding, softfloat)  |

## Target maintainers

[@jonathanpallant](https://github.com/jonathanpallant)

## Requirements

This target is cross-compiled. There is no support for `std`. There is no
default allocator, but it's possible to use `alloc` by supplying an allocator.

Functions marked `extern "C"` use the [PowerPC EABI calling
convention](https://www.nxp.com/docs/en/application-note/PPCEABI.pdf).

This target generates ELF binaries. Any alternate formats or special
considerations for binary layout will require linker options or linker scripts.

## Building the target

You can build Rust with support for the target by adding it to the `target`
list in `bootstrap.toml`:

```toml
[build]
build-stage = 1
host = ["<target for your host>"]
target = ["<target for your host>", "powerpc-unknown-none-eabi"]
```

Replace `<target for your host>` with `x86_64-unknown-linux-gnu` or whatever
else is appropriate for your host machine. You may also need to set the CC variable:

```bash
export CC_powerpc_unknown_none_eabi=powerpc-none-eabi-gcc
```

## Building Rust programs

To build with this target, pass it to the `--target` argument, like:

```console
cargo build --target powerpc-unknown-none-eabi
```

This target uses LLD as a linker, but you can override this in your project configuration, as follows:

`.cargo/config.toml`:
```toml
[target.powerpc-unknown-none-eabi]
linker = "powerpc-custom-elf-gcc"
```

## Testing

As `powerpc-unknown-none-eabi` supports a variety of different environments and does
not support `std`, this target does not support running the Rust test suite.

You can test `#![no_std]` Rust binaries generated with this target using QEMU:

```bash
qemu-system-powerpc -machine ppce500 -cpu e500 -d guest_errors,unimp -nographic -bios your_binary.elf 
```
