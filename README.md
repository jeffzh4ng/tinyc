# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> RISC-V

`din`'s primary goals are academic. This design point is reflected in 1. the
pedagogic nature of architecture documents and 2. using toy programs as benchmarks,
instead of real workloads.

### Architecture
- [ARCHITECTURE](./ARCHITECTURE)
- [COMPILER](./COMPILER)
- [CHIP](./CHIP)

### Benchmarks
|                     | din  | gcc | clang |
| -------------       | ---- | --- | ----  |
| fannkuch-redux      |      |     |       |
| n-body              |      |     |       |
| spectral-norm       |      |     |       |
| mandelbrot          |      |     |       |
| pidigits            |      |     |       |
| regex-redux         |      |     |       |
| fasta               |      |     |       |
| k-nucleotide        |      |     |       |
| reverse complement  |      |     |       |
| binary trees        |      |     |       |

Source: https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/c.html

### Roadmap
- [ ] interpreting C
- [ ] compiling to RISC-V, ran on third party RISC-V simulator
- [ ] compiling to RISC-V, ran on Rust-implemented RISC-V simulator
- [ ] compiling to RISC-V, ran on Verilog-implemented RISC-V FPGA

[^0]: like Google search, for example