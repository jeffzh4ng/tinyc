# Architecture

**Contents**
1. [Frontend](./https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#frontend-lexing-parsing-typing)
    - [Formalizations](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#formalizations)
    - [Parsing](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#parsing-top-down-recursive-descent)
2. [Backend]()
3. [References: Interpreters and Compilers](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-interpreters-and-compilers)

4. [References: Source and Target Languages](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-source-and-target-languages)
    - [Source: C89/90](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#source-c8990)
    - [Target 1: RISC-V](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#target-1-risc-v)
    - [Target 2: LLVM](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#target-2-llvm)

# Frontend (lexing, parsing, typing)
```
chars -> |lexer| -> tokens -> |parser| -> parse tree -> |elaborator| -> abstract syntax tree
```

din's frontend follows a traditional three pass architecture, where separation
of concerns is split based on the different levels of abstraction which naturally
occur when raising the representation of source from charactersk, to tokens, to
trees.

While academia tends to formalize both lexical and syntactic analysis with
well-defined compiler compilers, din's lexer and parser are both handwritten.
There are even many open source compilers such as GCC and Clang which
handwrite their own frontends; and din follows suit. However, if you want
a quick overview of the theory, feel free to expand the section below. Otherwise,
we will move on with Pratt Parsing.

### Formalizations
<details>
  <summary>Expand</summary>
Lexing and parsing sit on a strong foundation of theory which sit at the
intersection of languages and computation. The core problem of both lexical and
syntactic analysis is to recognize a series of symbols from an alphabet by
producing a derivation of productions specified by the language. That was a bunch
of word salad according to formal definitions. More clearly:

Lexical analysis:
- alphabet: characters
- series of symbols: tokens
- language: regular
  - spec: regular expressions (RE)
  - impl: deterministic finite automata (DFA)

Syntactic analysis:
- alphabet: tokens
- series of symbols: tree
- language: context-free
  - spec: Backus-Naur Form (BNF)
  - impl: pushdown automata (PA)

There are well-defined algorithms to convert specs into implementations. For
instance, with syntactic analysis, you can convert REs -> NFAs -> DFA -> min(DFA)
via Thompson's, subset, and Kleene's construction, respectively.

RE/FA aren't expressive enough for a certain set of languages, called context-free
languages (in which you can use the Pumping Lemma result to determine if
a language is regular or not). If a language is context-free, it'll need to be
specified via BNF and implemented with pushdown automata, which are the same
as their regular-language counterparts with the addition of recursion.

The formalization of both lexical and syntactic analysis results in so called
compiler compilers which take in your lexical and syntactic grammars, and produce
the machines (lexers and parsers), which *you* then use for your compiler. This
is not so different from higher order programming.

While these academic formalizations can help compiler construction with respect
to correctness (rule #1 of compiler construction is to perserve semantics
afterall), caution should be exercised based on your engineering constraints.
A heuristic to use when calculating cost-benefit calculus is `benefit (DSL) ∝ |engineers|`
Sacrificing flow control for a straight-jacketed DSL (such as HCL and ECS for
managing cloud infrastructure and building games) makes sense when
`|engineers| > 1e4`, but definitely not for a project like din, where
`|engineers| = 1`.

The only theory din leverages is the research behind the different types of
top down parsing (recursive descent) to handle operation precedence and
associativity with non-Lisp-like-S-expression-syntax, which, so happens to be
din's case, as its source language is C.

</details>

### Parsing: top-down, recursive descent

a literal translation of the grammar’s rules straight into imperative code.

*References: Pratt Parsing (the monads of syntac analysis)*
[index](https://www.oilshell.org/blog/2017/03/31.html)

*ogs*
- [Dijkstra (1961)](https://ir.cwi.nl/pub/9251/9251D.pdf)
- [Pratt (1973)](https://tdop.github.io/)
- [Norvell (1999)](https://www.engr.mun.ca/%7Etheo/Misc/exp_parsing.htm)

*new gen*
- [Crockford (2007)](https://crockford.com/javascript/tdop/tdop.html)
- [Bendersky (2010)](https://eli.thegreenplace.net/2010/01/02/top-down-operator-precedence-parsing)
- [Nystrom (2011)](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
- [Ball (2016)](https://edu.anarcho-copy.org/Programming%20Languages/Go/writing%20an%20INTERPRETER%20in%20go.pdf)
- [Kladov (2020)](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing)

Recursive descent ⊆ Pratt Parsing ≅ Shunting Yard
- [Chu (2016)](https://www.oilshell.org/blog/2016/11/01.html)
- [Chu (2017)](https://www.oilshell.org/blog/2017/03/30.html)
- [Norvell (2016)](https://www.engr.mun.ca/%7Etheo/Misc/pratt_parsing.htm)
- [Kladov (2020)](https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html)
- [Johnston (2021)](https://www.abubalay.com/blog/2021/12/31/lr-control-flow)

# References: Interpreters and Compilers
**Interpreters and Compilers**
- Programming Languages: Application and Interpretation (Krishnamurthi)
- Engineering a Compiler (Cooper, Torczon)
- [Cornell's CS 4120 SP23 Lecture Notes (Myers)](https://www.cs.cornell.edu/courses/cs4120/2023sp/notes/)

note: please avoid the dragon book. You'll walk away with the impression that
compiler construction is primarily about parsing (and not much on other important
areas, like type checking, optimization, etc), when in fact parsing should
be the easiest part of the compiler.

**Optimizations**
- 80s: register allocation
- 90s: scheduling (bc RISC introduced pipelining)
- instruction selection?

# References: Source and Target Languages

### Source: C89/90
- [C Standards (Drafts)](https://github.com/sys-research/c-standard-drafts)
- The C Programming Language (K&R)
- If You Must Learn C (Ragde)

*Lexical grammar*
```
// introductions

LITERAL_INT      ::= [0-9]+
ID               ::= [a−zA−Z][a−zA−Z0−9]*

// keywords
KEYWORD_INT      ::= int
KEYWORD_VOID     ::= void
KEYWORD_RETURN   ::= return

// eliminations
PLUS             ::= +
MINUS            ::= -
STAR             ::= *
SLASH            ::= /

// punctuation
PUNC_LEFTPAREN   ::= (
PUNC_RIGHTPARE   ::= )
PUNC_LEFTBRACE   ::= {
PUNC_RIGHTBRAC   ::= }
PUNC_SEMICOLON   ::= ;
```

*Syntactical grammar*
```
<program>        ::= <function>
<function>       ::= KEYWORD_INT <identifier> PUNC_LEFTPAREN KEYWORD_VOID
                     PUNC_RIGHTPAREN PUNC_LEFTBRACE <statement> PUNC_RIGHTBRACE

<statement>      ::= KEYWORD_RETURN <exp> PUNC_SEMICOLON
<exp>            ::= LITERAL_INT
                   | <exp> <binop> <exp>
                   | PUNC_LEFTPAREN <expr> PUNC_RIGHTPAREN

<binop>          ::= PLUS
                   | MINUS
                   | STAR
                   | SLASH

<!-- <val> ::= literalint -->
```

*Semantics (types)*
### Target 1: RISC-V
- The RISC-V Reader (Waterman, Patterson)
- Computer Organization and Design RISC-V Edition: The Hardware Software Interface (Patterson, Hennessy)
- Computer Architecture: A Quantitative Approach (Hennesey, Patterson)
- Digital Design and Computer Architecture (Harris, Harris)
- Inside the Machine (Stokes)

### Target 2: LLVM
- [LLVM for Grad Students (Sampson)](https://www.cs.cornell.edu/~asampson/blog/llvm.html)
- [Greenplace (Bendersky)](https://eli.thegreenplace.net/tag/llvm-clang)
- [Compilers and IRs (Zhang)](https://www.lei.chat/posts/compilers-and-irs-llvm-ir-spirv-and-mlir/)
- [AOSA: LLVM (Lattner)](https://aosabook.org/en/v1/llvm.html)
- [Tourist's Guide to the LLVM Source Code (Regehr)](https://blog.regehr.org/archives/1453)