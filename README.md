# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> RV32I

`din`'s primary goals are academic. The first goal is to beat `gcc -O1` in the
toy benches provided below. The second goal is to compile real-world software
such as git, linux, and sqlite. din won't be compiling itself since it's hosted
in Rust.

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
- [ ] compiling to RV32I, ran on Spike
  - [ ] arithmetic
  - [ ] arithmetic, control flow
  - [ ] arithmetic, control flow, functions
  - [ ] arithmetic, control flow, functions, malloc/free
  - [ ] arithmetic, control flow, functions, malloc/free, beat `gcc -O1`
- [ ] compiling to RV32I, ran on HiFive QEMU
- [ ] compiling to RV32I, ran on HiFive metal
- [ ] git
- [ ] sqlite
- [ ] linux