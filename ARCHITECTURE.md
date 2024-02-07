# Architecture
Some thoughts on the theory behind languages, interpreters, compilers, and how
it translates (if at all) to din's architecture.

**Contents**
1. [Frontend](./https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#frontend-lexing-parsing-typing)
2. [Backend]()
3. [References: Interpreters and Compilers](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-interpreters-and-compilers)
4. [References: Source and Target Languages](https://github.com/jeffzh4ng/din/blob/master/ARCHITECTURE.md#references-source-and-target-languages)


TODO: preprocessor

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
Many open source compiler frontends follow suit, such as GCC and Clang.

A heuristic the author used for calculating cost-benefit calculus is `benefit (DSL) ∝ |engineers|`

Sacrificing flow control for a straight-jacketed DSL (such as HCL and ECS for
managing cloud infrastructure and building games) may make sense when
`|engineers| > 1e4`, but definitely not for a project like din, where
`|engineers| = 1`.

### 1. Lexing
```rust
struct Token {
    pub lexeme: String,
    pub category: Category, // literals, identifiers, keywords, punctuation, etc.
}

struct Lexer {}

impl Lexer {
   pub fn scan(input: Vec<char>) -> Vec<Token> {}
}

```

TODO: A is the language of M = M recognizes A = A = L(M)


For lexing, the central problem is recognizing tokens from
characters. Intuitions with formal models can start either via specification via
regular expressions (REs) or implementation via finite automata (FA).

Regardless of entry point, you'll find these models are equivalent by converting
REs -> NFAs -> DFA -> REs via Thompson's, subset, and Kleene's construction
respectively. These well-defined formalisms and their correctness properties lend
themselves to lexer compilers such as Lex and Flex which take REs as input, and
produce table-driven lexers as output.

Due to the cost benefit analysis stated above, din ignores lexer compilers. Its
lexer is hand-written.

### 2. Parsing

*Specifications: Grammars*
For parsing and syntactic analysis, REs are not strong enough to express the
recursive-like nature of expressions declared in a non-lisp-S-expression-like
syntax. Consider

```TODO
(a + b) * c
```

This is because REs and their DFA counterparts don't
have the necessary state to balance parentheses.
TODO: pumping lemma...

---

In formal language theory, this
motivates to move one step up the
[Chomsky Hierarchy](https://en.wikipedia.org/wiki/Chomsky_hierarchy) to
context-free grammars (CFGs), and their specifications via Backus-Naur form (BNF).

The formal model is similar in form to lexing. Given a token stream, the parser
needs to build a constructive proof that the token stream can be derived from
a grammar's derivations.

CFGs, at their core: are REs with recurison.
- derivations (list of rules) create productions (trees) with non-terminals and terminals.
- BNF specification

---

```rust
struct Parser

impl Parser {
  fn parse(tokens: Vec<Token>) -> Expr {

  }
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