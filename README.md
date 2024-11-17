# rSentenceHash

rSentenceHash is a for-fun project for finding a sentence where the last 9 character of the sentence's SHA256 hash
is appended in the sentence itself.

# Usage

Simply pull the repo, and run/build the binary with cargo.

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
