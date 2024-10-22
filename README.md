# tinyc
![](./stallman.jpg)

**a software 1.0 compiler: C89/90 -> RV32G**

`tinyc` is an batch-style compiler that compiles the unsafest language with the safest.

tinyc is smoked with egos2k and xv6 and passes gcc's torture test suite.
tinyc is sped up with 20/80 optimizations to beat `gcc -O1` on dhrystone and embench.
proebsting's law prevents me from beating -O2/-O3. check out my software 2.0 compiler.