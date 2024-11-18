# rSentenceHash

![minimum rustc 1.63](https://img.shields.io/badge/rustc-1.63+-red.svg)
![GitHub License](https://img.shields.io/github/license/thetruecolonel/rSentenceHash)


rSentenceHash is a for-fun project for finding a sentence where the last 9 character of the sentence's SHA256 hash
is appended in the sentence itself.

# Installation

### Ubuntu

```shell
sudo apt update
sudo apt install build-essentials sccache git
```

### Arch Linux

```shell
sudo pacman -Syu
sudo pacman -S base-devel sccache git
```

### All Platforms

- Install Rust ([Docs][rust-install])

```shell
git clone https://github.com/TheTrueColonel/rSentenceHash.git
cd rSentenceHash
cargo run --release
```

# Performance

> Data gathered by my own machines.

- CPU: **AMD Ryzen 5800X3D**
- OS: **Arch Linux**
- Kernel: **6.11.8-zen1-2-zen**
- rustc: **1.81.0**

| Version | Iterations     | Duration | Iterations per Second | Cores |
|---------|----------------|----------|-----------------------|-------|
| 1.0.0   | 48,405,995,370 | 918.937s | ~ 52.676 MIPS         | 12    |
| 1.1.0   | 48,405,995,370 | 387.078s | ~ 125.055 MIPS        | 12    |


- CPU: **Intel Xeon E5-2697 v2 x2**
- OS: **ProxMox VE 8.2.7**
- Kernel: **6.8.12-2-pve**
- rustc: **1.81.0**

| Version | Iterations     | Duration   | Iterations per Second | Cores |
|---------|----------------|------------|-----------------------|-------|
| 1.0.0   | 48,405,995,370 | 4,513.699s | ~ 10.724 MIPS         | 36    |
| 1.1.0   | 48,405,995,370 | 1,458.301s | ~ 33.197 MIPS         | 36    |

[rust-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html
