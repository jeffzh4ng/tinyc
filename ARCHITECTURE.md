# Architecture
Some thoughts on the theory behind languages, interpreters, compilers, and how
it translates (if at all) to Din's architecture.

TODO: preprocessor

### Frontend (lexing, parsing, typing)
Din's frontend follows a traditional three pass architecture, where separation
of concerns is split based on the different levels of abstraction which naturally
occur when raising the representation of source from characters, to tokens, to
trees.

While academia tends to formalize both lexical and syntactic analysis with
well-defined compiler compilers, Din's lexer and parser are both handwritten.
Many open source compiler frontends follow suit, like GCC and Clang.

A heuristic the author used for calculating cost-benefit calculus waas:
```
benefit(DSL) ∝ |engineers|
```

Sacrificing flow control for a straight-jacketed DSL (such as HCL and ECS for
managing cloud infrastructure and building games) may make sense when
`|engineers| > 1e4`, but definitely not for a project like Din, where
`|engineers| = 1`.

**1. Lexing**

For lexing, the central problem is recognizing tokens from
characters. Intuitions with formal models can start either via specification via
regular expressions (REs) or implementation via finite automata (FA).

Regardless of entry point, you'll find these models are equivalent by converting
REs -> NFAs -> DFA -> REs via the Thompson's, subset, and Kleene's construction
respectively. These well-defined formalisms and their correctness properties lend
themselves to lexer compilers such as Lex and Flex which take REs as input, and
produce lexers as output.

Due to the cost benefit analysis stated above, Din ignores lexer compilers. Its
lexer is hand-written.

**2. Parsing**
Pratt Parsing (aka the monads of syntactic analysis)
- [index](https://www.oilshell.org/blog/2017/03/31.html)
- [Dijkstra (1961)](https://ir.cwi.nl/pub/9251/9251D.pdf)
- [Pratt (1973)](https://tdop.github.io/)
- [Norvell (1999)](https://www.engr.mun.ca/%7Etheo/Misc/exp_parsing.htm)
- [Crockford (2007)](https://crockford.com/javascript/tdop/tdop.html)
- [Bendersky (2010)](https://eli.thegreenplace.net/2010/01/02/top-down-operator-precedence-parsing)
- [Nystrom (2011)](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
- [Ball (2016)](https://edu.anarcho-copy.org/Programming%20Languages/Go/writing%20an%20INTERPRETER%20in%20go.pdf)
- [Kladov (2020)](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing)

Comparisons
- [Oilshell (2016)](https://www.oilshell.org/blog/2016/11/01.html)
- [Norvell (2016)](https://www.engr.mun.ca/%7Etheo/Misc/pratt_parsing.htm)
- [Kladov (2020)](https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html)
- [Johnston (2021)](https://www.abubalay.com/blog/2021/12/31/lr-control-flow)

# References: Interpreters and Compilers
[An Incremental Approach to Compiler Construction (Ghuloum)](http://scheme2006.cs.uchicago.edu/11-ghuloum.pdf)

Note: please avoid the dragon book. You'll walk away with the impression that
compiler construction is primarily about parsing, when in fact parsing should
take no more than 5%-10% of total compile time.

**Interpreters and introductions**
- Programming Languages: Application and Interpretation (Krishnamurthi)
- Crafting Interpreters (Nystrom)
- Writing an Interpreter in Go (Ball)

**Compilers and formalisms**
- Writing a Compiler in Go (Ball)
- Engineering a Compiler (Cooper, Torczon)
- Modern Compiler Design (Grune)

**Optimizations**
- 80s: register allocation
- 90s: scheduling (bc RISC introduced pipelining)

# References: Source and Target Languages
**Source: C89**
- C99 ISO Standard
- If You Must Learn C (Ragde)
- The C Programming Language (K&R)
- C Programming: A Modern Approach (King)

**Target: RISC-V**
- The RISC-V Reader (Waterman, Patterson)
- Computer Organization and Design RISC-V Edition: The Hardware Software Interface (Patterson, Hennessy)
- Computer Architecture: A Quantitative Approach (Hennesey, Patterson)
- Digital Design and Computer Architecture (Harris, Harris)
- Inside the Machine (Stokes)