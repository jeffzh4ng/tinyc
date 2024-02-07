# Architecture

**Contents**
1. [Frontend](./https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#frontend-lexing-parsing-typing)
2. [Backend]()
3. [References: Interpreters and Compilers](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-interpreters-and-compilers)
4. [References: Source and Target Languages](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-source-and-target-languages)

# Frontend (lexing, parsing, typing)
```
chars -> |lexer| -> tokens -> |parser| -> parse tree -> |elaborator| -> abstract syntax tree
```

din's frontend follows a traditional three pass architecture, where separation
of concerns is split based on the different levels of abstraction which naturally
occur when raising the representation of source from characters, to tokens, to
trees.

While academia tends to formalize both lexical and syntactic analysis with
well-defined compiler compilers, din's lexer and parser are both handwritten.
There are even many open source compiler frontends such as GCC and Clang which
handwrite their frontends; and din follows suit. However, a quick overview of
the formalizations is given below.

**Formalizations**

Lexing and parsing sit on a strong foundation of theory which sets at the
intersection of languages and computation. The core problem of both lexical and
syntactic analysis is to recognize a series of symbols from an alphabet by
producing a derivation of productions specified by the language.

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

There are well-defined algorithms to convert specs into implementations, as well
as proofs (Pumping Lemma) for determining when a language is regular or not. For
instance, with syntactic analysis, you can convert REs -> NFAs -> DFA -> min(DFA)
via Thompson's, subset, and Kleene's construction respectively. This results
in so called "compiler compilers" which take in your lexical and syntactic
grammars, and produce the machines (lexers and parsers), which *you* then use
for your compiler. This is not any different from higher order programming.

While these academic formalizations can help compiler construction with respect
to correctness, caution should be exercised based on your engineering constraints.
A heuristic to use when calculating cost-benefit calculus is `benefit (DSL) ∝ |engineers|`
Sacrificing flow control for a straight-jacketed DSL (such as HCL and ECS for
managing cloud infrastructure and building games) may make sense when
`|engineers| > 1e4`, but definitely not for a project like din, where
`|engineers| = 1`.

The only theory din leverages is the research behind the different types of
top down parsing (recursive descent) to handle operation precedence and
associativity with non-Lisp-like-S-expression-syntax, which, so happens to be
din's case, as its source language is C.

### 1. Lexing
```rust
struct Token {
    pub lexeme: String,
    pub category: Category, // literals, identifiers, keywords, punctuation, etc.
}

pub fn scan(input: Vec<char>) -> Vec<Token> {}
```

### 2. Parsing
```rust
fn parse(tokens: Vec<Token>) -> Expr {

}
```
*Implementations: Top down (recursive descent)*

Top down refers to direction of tree creation via recursion. Recursive descent refers to
direction of grammar.

RECURSIVE DESCENT
- a literal translation of the grammar’s rules straight into imperative code.
problem 1: left recursion
problem 2: associativity
problem 3: efficiency

RECURSIVE DESCENT WITH OPERATOR PRECEDENCE (pratt or shunting)

*Bottom up (recursive ascent)*

Bottom up refers to direction of tree creation via recursion. Recursive ascent refers to
direction of grammar.

Yacc, Bison, ANTLR


Pratt Parsing (aka the monads of syntactic analysis)
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
compiler construction is primarily about parsing, when in fact parsing should
take no more than 5%-10% of total compile time.

**Optimizations**
- 80s: register allocation
- 90s: scheduling (bc RISC introduced pipelining)

# References: Source and Target Languages

### Source: C89
- [C Standards (Drafts)](https://github.com/sys-research/c-standard-drafts)
- The C Programming Language (K&R)
- If You Must Learn C (Ragde)

*Lexical grammar*
```
// introductions

literalint      ::= [0-9]+
id              ::= [a−zA−Z][a−zA−Z0−9]*

// keywords
keywordint      ::= int
keywordvoid     ::= void
keywordreturn   ::= return

// eliminations
plus            ::= +
minus           ::= -
star            ::= *
slash           ::= /

// punctuation
puncleftparen   ::= (
puncrightparen  ::= )
puncleftbrace   ::= {
puncrightbrace  ::= }
puncsemicolon   ::= ;
```

*Syntactical grammar*
```
<program>       ::= <function>
<function>      ::= keywordint <identifier> puncleftparen punckeywordvoid
                    puncrightparen puncleftbrace <statement> puncrightbrace

<statement>     ::= keywordreturn <exp> puncsemicolon
<exp>           ::= literalint
                | <exp> binop <exp>
                | puncleftparen <expr> puncrightparen

<binop>         ::= plus
                | minus
                | star
                | slash

<!-- <val> ::= literalint -->
```

*Semantics (types)*

### Target: RISC-V
- The RISC-V Reader (Waterman, Patterson)
- Computer Organization and Design RISC-V Edition: The Hardware Software Interface (Patterson, Hennessy)
- Computer Architecture: A Quantitative Approach (Hennesey, Patterson)
- Digital Design and Computer Architecture (Harris, Harris)
- Inside the Machine (Stokes)