# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> RV32I

`din` is an ambitiously academic batch-style compiler for a 0.25 man year of work.
There are three goals.

The first goal is to reach a base level of confidence in compiler correctness
by compiling [egos2000](), [xv6](), [git](), [sqlite](), and as much of `chibicc`,
`tcc`, and `gcc`'s test suite as possible. Linux is left as a challenge for when the
next Apple wants me to hire me for the third C/C++ compiler after gcc and clang.

The second goal is to reach a base level of performance by beating `gcc -O1` on
dhrystone and embench with similar optimizations as `qbe`. SPEC is left as another
challenge for employment, because I ain't paying for that out pocket.

The third (and long-term) goal is hacking on the longtail of correctness and
performance with csmith/yarpgen fuzzing, and -O2, and -O3 techniques, respectively.
However, because of Proebsting's Law, my efforts will probably shift to hacking
on software 2.0 compilers. Gotta follow those exponentials people!

### Roadmap
- [ ] 0.1 arithmetic, control flow, functions, malloc/free

- [ ] 0.2.1 chibicc test suite
- [ ] 0.2.2 tcc test suite
- [ ] 0.2.3 gcc test suite

- [ ] 0.3.1 egos2000
- [ ] 0.3.2 xv6
- [ ] 0.3.3 git
- [ ] 0.3.4 sqlite

- [ ] 0.4.1 assembler
- [ ] 0.4.2 linker

- [ ] 0.5.1 csmith, yarpgen
- [ ] 0.5.2 ssa

### Research
- [SOURCE](./docs/SOURCE)
- [TARGET](./docs/TARGET)
- [UARCH](./docs/UARCH)