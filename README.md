# tdos

A small toy kernel, written in Rust, based on [this blog](https://os.phil-opp.com/).

## Building

In order to build it, generally you need:

- `cargo` (the Rust build tool)
- `cargo-bootimage` (for creating bootable disk images)
- The Rust component `llvm-tools-preview`

Everything else is being handled by `cargo` itself.
Also note that, if you use Nix, than the `flake.nix` in this project already pulls in everything you need.

You can now create a bootimage by running `cargo bootimage` at the root of this project.
This will build a `bootimage-tdos.bin` at `target/x86_64-tdos/debug` which you can run in `qemu` using:

```sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-tdos/debug/bootimage-tdos.bin
```

Alternatively, given that you have `cargo-bootimage` and `qemu` installed, the `cargo run` command has been modified to
run the `cargo bootimage runner` command, which builds the bootimage and runs the `qemu` command mentioned above.
