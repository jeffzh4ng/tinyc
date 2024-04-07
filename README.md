# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> RV32I

`din` is ambitiously academic for a half person year of work, with three goals.

The first goal is to reach a base level of confidence in compiler correctness
by compiling [egos2000](), [xv6](), [git](), [sqlite](), and as much of the
gcc torture test suite as possible. Linux is left as a challenge for when the
next Apple wants to pay me for the next C/C++ compiler after gcc and clang.

The second goal is to reach a base level of performance by beating `gcc -O1` on
dhrystone and embench. SPEC is left as another challenge for employment, because
I ain't paying for that out pocket.

The third (and long-term) goal is to conduct further research into correctness
with fuzzing and verification, as well as performance with -O2, and -O3 optimization
techniques. However, because of Proebsting's Law, my efforts will probably shift 
to hacking on software 2.0 compilers. Gotta follow those exponentials people!

### Research
- [COMPILER](./COMPILER)
- [ISA](./ISA)
- [UARCH](./ARCHITECTURE)

### Roadmap
- [ ] correctness
  - [ ] arithmetic
  - [ ] arithmetic, control flow
  - [ ] arithmetic, control flow, functions
  - [ ] arithmetic, control flow, functions, malloc/free
  - [ ] arithmetic, control flow, functions, malloc/free, gcc torture
  - [ ] arithmetic, control flow, functions, malloc/free, gcc torture, egos2000
  - [ ] arithmetic, control flow, functions, malloc/free, gcc torture, egos2000, xv6
- [ ] performance
  - [ ]
- [ ] toolchain
  - [ ] assembler
  - [ ] linker
  - [ ] loader