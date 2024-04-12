# din
![](./din.gif)

a software 1.0 compiler: C89/90 -> RV32I

`din` is an ambitiously academic batch-style compiler for a 0.25 man year of work
that compiles the unsafest language with the safest. There's three goals:

The first goal is to reach a base level of confidence in compiler correctness
by compiling [egos2000](), [xv6](), [git](), [sqlite](), and as much of
[c-testsuite](https://github.com/c-testsuite/c-testsuite) as possible. Linux is
left as a challenge for when the next Apple wants to hire me for the third C/C++
compiler after `gcc` and `clang`.

The second goal is to reach a base level of performance by beating `gcc -O1` on
dhrystone and embench with similar optimizations as `qbe`. SPEC is left as another
challenge for employment, because I ain't paying for that out pocket.

The third (and long-term) goal is hacking on the longtail of correctness and
performance with csmith/yarpgen fuzzing, and -O2/-O3 techniques. However,
because of Proebsting's Law, my efforts will probably shift to hacking on
software 2.0 compilers. Gotta follow those exponentials people!

### Roadmap
A project's roadmap to 1.0 is great legend. Those who apply Sturgeon's Law onto
themselves say there's no sign of that road, let alone civilization. Those who
remain hopeful say it has always existed — you just need to search hard enough.
With that said, `din` will be 1.0 when it can compile Linux. Until then, it will
be zero versioned with [zer0ver](https://0ver.org/).

### Research
- [SOURCE](./examples/SOURCE)
- [INTER](./examples/INTER)
- [TARGET](./examples/TARGET)
- [UARCH](./examples/UARCH)

### References
- Cooper, Torczon
- Muchnick
- Levine
- Patterson & Hennesey
- Hennesey & Patterson