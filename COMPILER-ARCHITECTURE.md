# Compiler Architecture

**Contents**
1. [Frontend (Parsing)](#1-frontend)
  - A. [Lexical and Syntactic Analysis: Lexing and Parsing](#a-lexical-and-syntactic-analysis-lexing-and-parsing)
  - B. [Source Language References: C89/90](#b-source-language-references-c8990)
  - C. [Semantic Analysis: Typing](#c-semantic-analysis-typing)
2. [Middleend (Optimization)]
3. [Backend (Code Gen)](#2-backend)
  - A. Selection
  - B. Scheduling
  - C. Allocation
  - D. [Target Language 1 References: LLVM](#d-target-language-1-llvm)
  - E. [Target Language 2 References: RISC-V](#e-target-language-2-risc-v)
4. [References](#3-references)
  - [A. Languages and Compilers](#a-languages-and-compilers)

# 1. Frontend (Parsing)
```
chars -> |lexer| -> tokens -> |parser| -> tree -> |typer| -> typed tree
```

din's frontend follows a traditional three pass architecture, where separation
of concerns is split based on the different levels of abstraction that naturally
occurs when raising the representation of source from characters, to tokens, to
trees.

While academia formalizes both lexical and syntactic analysis with well-defined
"compiler compilers", din's lexer and parser are both handwritten. There are even
many open source compilers such as GCC and Clang which handwrite their own frontends;
and din follows suit. However, if you want a quick overview of the theory, feel
free to expand the section below. Otherwise, we will move on with Pratt Parsing.

## A. Lexical and Syntactic Analysis: Lexing and Parsing

### Academic Parsing: formalizations with automata
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

### Practical Parsing: top-down, recursive descent

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


## B. Source Language References: C89/90
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

## C. Semantic Analysis: Typing

---

# 2. Middleend (Optimization)
why?
  - easier optimization
  - m*n --> m+n

high-level irs (trees)
  - optimizations close to source
  - useful for semantic analysis (type checking)
  - error messages. that make sense with concrete syntax :)

medium-level irs
  - gcc: 3AC --> ???
  - llvm: CFG --> ???

  - quadrubles: 3 address a = b OP c
  - triples: 2 address OP a b (dest is implicit)
  - double: ???
  - single: ???
  - zero: 0 address (stack) jvm bytecode for instance
    - jvm is a stack-based machine (no registers)
    - easy b/c you don't have to worry about register allocation
  
  - simple let language (SLL)
    - intermediate values (never modified)
  --> order of eval is explicit

  - llvm (typed SSA. like ml assignment. no mutation)
    - offers human readable textual rep (.ll) (target 2 is .ll)
    - you'll have CFG in memory, but then output .ll
    - CFG is a list of labeled basic blocks
      - no two blocks have same label (invariant)
      - all terminators mention only labels defined
      - there is a distinguished, unlabeled, entry block

    - ll storage models: locals
    --> LL is a VM. essentially a an AFSM too.

low-level irs (close to assembly. pseudo assembly)
  - optimizations based on target architecture
  - you don't want to think about objects at the time
    you are dealing with instruction selection
    (well, maybe if your ISA is CISC with low semantic gap)
    --> depends. what are the design points (onur)

  - selection
  - scheduling
  - allocation

---

# 3. Backend (Code Gen)
- scheduling and allocation are NP-complete.
- ISA
  - tradeoffs (collection of features. risc vs cisc)
  - we are compiling to risc --> semantic gap is large
  --> lots of room for compiler optimizatins?
  - more in [CHIP-ARCHITECTURE.md](./CHIP-ARCHITECTURE.md)

## A. Selection
## B. Scheduling
- (order of operations, so processor can keep pipelines full)
## C. Allocation (register)

## D. Target Language 1: RISC-V
- The RISC-V Reader (Waterman, Patterson)

## E. Target Language 2: LLVM
- [LLVM for Grad Students (Sampson)](https://www.cs.cornell.edu/~asampson/blog/llvm.html)
- [Greenplace (Bendersky)](https://eli.thegreenplace.net/tag/llvm-clang)
- [Compilers and IRs (Zhang)](https://www.lei.chat/posts/compilers-and-irs-llvm-ir-spirv-and-mlir/)
- [AOSA: LLVM (Lattner)](https://aosabook.org/en/v1/llvm.html)
- [Tourist's Guide to the LLVM Source Code (Regehr)](https://blog.regehr.org/archives/1453)

# 3. References
Good compiler construction references for hackers who want to show up
[done](https://steve-yegge.blogspot.com/2008/06/done-and-gets-things-smart.html)
are few to none. They range from textbooks all about frontend formalizations such
as parsing (hello dragon book), to calculator blog posts which lack substance.

More importantly, simply generating one specific ISA and treating
the metal below you as a black box is not conducive towards successful compilers.
Often, you will be co-designing fullstack hardware/software systems with computer
architects, and so knowledge of ISA and microarchitecture will help you design
on based on principle rather than precedent.

Because of the vast amount of area to cover, there is not one single book which
contains all the gold. Knowledge from courses, books, and elder grey-beards must
be patchworked together. Clearly the last type of knowledge can only be passed
down within the trenches, but here are a few resources which I found not too bad.

![](./kaepora.webp)
> Hoo hoot! Link... Look up here! It appears that the time has finally come for
> you to start your adventure! You will encounter many hardships ahead... That
> is your fate. Don't feel discouraged, even during the toughest times!

## A. Languages and Compilers
- Programming Languages: Application and Interpretation (Krishnamurthi)
- Cornell CS 4120 SP23 Lecture Notes (Myers)
- Engineering a Compiler (Cooper, Torczon)

## B. Computer Architecture
- Computer Organization and Design RISC-V Edition: The Hardware Software Interface (Patterson, Hennessy)
- Digital Design and Computer Architecture, RISC-V Edition (Harris, Harris)