# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> {RISC-V/CUDA}

`din`'s primary goals are academic. The goal is to beat `gcc -O1` in the benches
provided below.

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
- [ ] compiling to RISC-V, ran on third party RISC-V simulator
  - [ ] arithmetic
  - [ ] arithmetic, control flow
  - [ ] arithmetic, control flow, functions
  - [ ] arithmetic, control flow, functions, malloc/free
  - [ ] arithmetic, control flow, functions, malloc/free, beat `gcc -O1`
    - [ ] strength reduction
    - [ ] register allocation
    - [ ] deadcode elimination
    - [ ] partial redundancy elimination
    - [ ] SSA
    - [ ] SIMD
- [ ] compiling to RISC-V, ran on Rust-implemented RISC-V simulator
- [ ] compiling to RISC-V, ran on Verilog-implemented RISC-V FPGA