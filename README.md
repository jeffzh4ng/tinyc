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
performance with csmith/yarpgen fuzzing, and -O2/-O3 techniques. However,
because of Proebsting's Law, my efforts will probably shift to hacking on
software 2.0 compilers. Gotta follow those exponentials people.

### Roadmap
A project's roadmap to 1.0 is great legend. Those who apply the Lindy Effect (or
conversely, Sturgeon's Law) onto themselves say there's no sign of that road, let
alone civilization. Those who remain hopeful say it has always existed — you just
need to search hard enough. With that said, `din` will be 1.0 when it can compile
Linux but until then, it will be zero versioned with [zer0ver](https://0ver.org/).

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