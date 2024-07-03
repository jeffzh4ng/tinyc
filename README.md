# din
![](./din.gif)

**a software 1.0 compiler: C89/90 -> RV32G**

`din` is an ambitiously academic batch-style compiler for a 0.25 man year of work
that compiles the unsafest language with the safest. There's three goals

The first goal is to reach a base level of confidence in compiler correctness
by smoking `din` with 2kloc/10kloc operating systems ([egos2000](), [xv6]()),
and stressing it with `gcc`'s torture test suite. Because the correctness needs
to be bootstrapped from zero, `din`'s passes will look like HLL -> AST -> 3AC
register machine (emulating 1AC stack machine). Compiling Linux will be left as
a challenge for when someone wants to hire me to help out with the third C/C++
compiler after `gcc` and `clang`.

The second goal, (after a base level of correctness has been achieved) is to
employ 20% of the optimizations that will give din 80% of the speed, in order
to beat `gcc -O1` on dhrystone and embench. `din`'s passes and representations
will be refactored to look like HLL -> AST -> CFG -> 3AC register machine. The
SPEC benchmarks will be left as another challenge for employment, because I ain't
paying for that out pocket.

The third (and long-term) goal is hacking on the longtail of correctness and
performance with csmith/yarpgen fuzzing, and -O2/-O3 techniques. A fun paper to
read is Thompson's [Reflections on Trusting Trust](https://www.cs.cmu.edu/~rdriley/487/papers/Thompson_1984_ReflectionsonTrustingTrust.pdf), and
Wheeler's [Countering Trusting Trust](https://dwheeler.com/trusting-trust/),
which is the compiler-equivalent of math and computation's Incompleteness and
Undecidable results. However, because of Proebsting's Law, my efforts will
probably shift to hacking on software 2.0 compilers.

### Research
- [SOURCE](./docs/SOURCE)
- [INTER](./docs/INTER)
- [TARGET](./docs/TARGET)
- [UARCH](./docs/UARCH)

### References
**Textbooks**
- Cooper, Torczon
- Muchnick
- Harper
- SSA book
- Levine
- Patterson & Hennesey

**Architectures**
- [rustc High Level Compiler Architecture](https://rustc-dev-guide.rust-lang.org/part-2-intro.html)
- [rustc Source Code Representation](https://rustc-dev-guide.rust-lang.org/part-3-intro.html)
- [The Architecture of Open Source Applications: LLVM](https://aosabook.org/en/v1/llvm.html)
- [The Conceptual Structure of GCC](https://www.cse.iitb.ac.in/grc/intdocs/gcc-conceptual-structure.html#The-GCC-IRs)
- [GCC Internals](https://gcc.gnu.org/onlinedocs/gccint/)