# PicoRV32

This project is an attempt to run picorv32 on my own FPGA board with customized
firmware (in Rust).

The verilog code for picorv32 and the top system is taken from [picorv32](https://github.com/YosysHQ/picorv32).

## Firmware

The firmware is written in Rust. The [rCore](https://rcore-os.cn/rCore-Tutorial-Book-v3/) tutorial helped me a lot to get started with the embedded Rust.

To build the firmware, follow the commands below:

```bash
# Add toolchain for RV32I
rustup target add riscv32i-unknown-none-elf
# Build with Cargo
cd firmware
cargo build --release
# Strip the binary
rust-objcopy --strip-all target/riscv32i-unknown-none-elf/release/firmware -O binary firmware.bin
# Use the `makehex.py` to generate memory initialization file
cd ..
python makehex.py firmware/firmware.bin 4096 > firmware.hex
```

The script `makehex.py` is also taken from the [picorv32](https://github.com/YosysHQ/picorv32).

## Development

In VSCode, the following settings are necessary for rust-analyzer to check the firmware code:

```json
{
    "rust-analyzer.linkedProjects": [
        "${workspaceFolder}/hello/firmware/Cargo.toml"
    ],
    "rust-analyzer.cargo.allTargets": false,
    "rust-analyzer.cargo.extraArgs": [
        "--target",
        "riscv32i-unknown-none-elf"
    ]
}
```

The constraint file is `system.xdc`, which is for [Alinx AX7035B](https://www.en.alinx.com/Product/FPGA-Development-Boards/Artix-7/AX7035B.html). You may need to modify it for your own board.

To check the output bytes of the firmware, an Integrated Logic Analyzer (ILA) is used in Vivado. I
set up the ILA with post-synthesis netlist, by setting up the debug ports in the top module[^1].

[^1]: An [article](https://www.cnblogs.com/WenGalois123/p/16565305.html) about the usage of ILA in Vivado.
