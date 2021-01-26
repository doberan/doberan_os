# doberan_os

## References

<https://os.phil-opp.com/ja/freestanding-rust-binary/>

## command

Preparation install

```bash
scoop install qemu
cargo install bootimage
rustup component add llvm-tools-preview
```

start qemu

```bash
qemu-system-x86_64.exe -drive format=raw,file=target\x86_64-doberan_os\debug\bootimage-doberan_os.bin
```
