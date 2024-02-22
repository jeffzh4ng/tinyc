# din
![](./din.gif)

a CPU compiler (C89/90 -> RISC-V); [a project of one's own](https://paulgraham.com/own.html)

### Architecture
- [SYSTEM-ARCHITECTURE.md](./ARCHITECTURE.md)
- [COMPILER-ARCHITECTURE.md](./COMPILER-ARCHITECTURE.md)
- [CHIP-ARCHITECTURE.md](./CHIP-ARCHITECTURE.md)

### Benchmarks
Although the best choice of benchmarks are real applications[^0] and avoiding
toy programs and synthetic benchmarks below, the purpose of this project is to learn
more about the design and implementation of compilers/chips that power general purpose workloads.

https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/c.html

- [ ] fannkuch-redux
- [ ] n-body
- [ ] spectral-norm
- [ ] mandelbrot
- [ ] pidigits
- [ ] regex-redux
- [ ] fasta
- [ ] k-nucleotide
- [ ] reverse complement
- [ ] binary trees

### Roadmap
- [ ] intepreting C
- [ ] compiling to RISC-V, ran on third party RISC-V simulator
- [ ] compiling to RISC-V, ran on Rust-implemented RISC-V simulator
- [ ] compiling to RISC-V, ran on Verilog-implemented RISC-V FPGA

[^0]: like Google search, for example