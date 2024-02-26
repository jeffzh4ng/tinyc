# Compiler Architecture
**Contents**

1. [Representations]
2. [Frontend (Parsing)](#1-frontend)
  - A. Academic Recognition: Formalizations with Automata
  - B. Lexical Analysis: Lexing
  - C. Syntactic Analysis: Parsing
  - D. Semantic Analysis: Type Checking
  - E. Correctness: Formal Verification
  - F. Source Language References (C89/90)
3. [Middleend (Optimization)]
4. [Backend (Code Gen)](#2-backend)
  - A. Selection
  - B. Scheduling
  - C. Allocation
  - D. [Target Language 1 References: LLVM](#d-target-language-1-llvm)
  - E. [Target Language 2 References: RISC-V](#e-target-language-2-risc-v)
5. [References](#3-references)
  - [A. Languages and Compilers](#a-languages-and-compilers)

# 1. Big Picture
The two golden rules of compiler construction (any computing system really), is:
1. correctness: preserve source semantics
2. performance: optimize if possible

<details>
<summary>Food for thought</summary>
Assuming Proebstings Law[^0], which is really being aware of:
1. compiler optimizations yield 4% speedup/year
2. rule of 72

then compiler-related optimizations yield a doubling in program speedup every
18 years. So perhaps there's more ROI to focus at the top[^1] and bottom[^2].
</details>

TODO: i will look at some of the major optimizations, but due to proebstings law,
compiler correctness seems to be a more fruitful activity, since din is being
used instrumentally as a research project.

Compiler construction is usually broken down into three phases: frontend, middleend
and backend. Just like how the more larger problem of translation between problems
and electrons is broken down, so is too the problem between high level language
and that of a machine.

At each phase of the compiler, the representation of the program changes form.
Usually, in the frontend, we represent the program as trees which helps with
recognition. In the middleend, control flow graphs help with optimization. And
finally, assembly helps with, well, the original problem! Generation of machine
language.

# 2. Frontend (Recognition)
```rust
fn lex(&[char]) -> Vec<Token>
fn parse(Vec<Token>) -> Tree
fn typ(Tree) -> bool
```

```
                                                                                              +
                                                                                             / \
 9 * 10 + 3 --->|LEXER|---> [Lit(9), Star(*), Lit(10), Plus(+), Lit(3)] ---> |PARSER| --->  *   3 ---> |TYPER| ---> true
                                                                                           / \
                                                                                          9  10
```

din's frontend follows a traditional three pass architecture, where separation
of concerns is split based on the different levels of abstraction that naturally
occurs when raising the representation of source from characters, to tokens, to
trees, to attributed trees.

While academia formalizes frontend recognition (lexical, syntactic, and semantic
analysis) with well-defined compiler compilers and ____, din's lexer, parser,
and type checker are hand-written.

Many open source compilers such as GCC and Clang which handwrite their own frontends;
and din follows suit. However, there are some golden nuggets of truth with well-defined
formalizations when it comes to compiler *correctness*. Golden rule of compiler
construction is to perserve semantics, afterall.

If you'd like a quick overview of the theory, feel free to expand section A below.
Otherwise, we will move on to section B with Pratt Parsing.

### A. Academic Recognition: Formalizations with Automata
<details>
<summary>Expand</summary>
Lexing, parsing, and typing sit on a strong foundation of computional theory. The
core problem of the compiler's frontend is to *recognize* a series of symbols (called
productions) from an alphabet by deriving a series of productions specified by a
grammar. You can think of recognition as *judgement*, and the series of productions
as *parsing*.

Lexical analysis:
- alphabet (input): characters
- productions (output): tokens
- language: regular
  - spec: regular expressions (RE)
  - impl: deterministic finite automata (DFA)

Syntactic analysis:
- alphabet (input): tokens
- productions (output): tree
- language: context-free
  - spec: Backus-Naur Form (BNF)
  - impl: pushdown automata (PA)

Semantic analysis
- alphabet (input): tree
- productions (output): attributed tree
- language: context-sensitive
  - spec: ??
  - impl: ??

There are well-defined algorithms to convert specs into implementations. For
instance, with syntactic analysis, you can convert REs -> NFAs -> DFA -> min(DFA)
via Thompson's, subset, and Kleene's construction (respectively).

RE/FA aren't *expressive* enough for a certain set of languages, called context-free
languages. There are fancy results such as the Pumping Lemma to determine if a
language is regular not. A more practical litmus test is to check if your parser
needs to recognize if a series of open and closed parentheses is balanced or not.
Intuitively, *finite automata* aren't strong enough to perform this analysis, as an
unbounded number of states is required.

This motivates the next jump up the Chomsky hierarchy, to context-free languages,
which are specified with BNF grammars and are implemented with *pushdown automata*.
They are similar to DFAs, with the addition of *recursion*, by adding references
to other productions within a production itself. These references are called
*non-terminals*, whereas the literal (regular) references are called *terminals*.

The formalizations of frontend analysis result in so called "compiler
compilers" which take in your lexical and syntactic grammars, and produce
the programs (lexers and parsers), which *you* then use for your compiler. This
is not so different from higher order programming.

TODO: type theory

- The grammar is called “context-free” because whether the production applies doesn't depend on the surrounding context α and β.


A heuristic to use when performing cost-benefit calculus is `benefit (DSL) ∝ |engineers|`
Sacrificing flow control for a straight-jacketed DSL (such as HCL and ECS for
managing cloud infrastructure and building games) makes sense when
`|engineers| > 1e4`, but definitely not for a project like din, where
`|engineers| = 1`.

The only theory din leverages is the research behind the different types of
top down parsing (recursive descent) to handle operation associativity and
precedence with non-Lisp-like-S-expression-syntax, which, so happens to be
din's case, as it's source language is C.
</details>

### B. Lexical Analysis: Lexing
Grammar:
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

- notice how this spec doesn't recurse --> it's regular
TODO: the hand-written implementation is what you would naturally derive
      from solving it yourself. no formal textbooks needed.

### C. Syntactic Analysis: Parsing
**Syntactic Grammar**
```
<program>        ::= <function>
<function>       ::= KEYWORD_INT KEYWORD_MAIN PUNC_LEFTPAREN KEYWORD_VOID
                     PUNC_RIGHTPAREN PUNC_LEFTBRACE <statement> PUNC_RIGHTBRACE

<statement>      ::= KEYWORD_RETURN <expr> PUNC_SEMICOLON
<expr>            ::= LITERAL_INT
                    | <expr> <binop> <expr>

<binop>          ::= PLUS
                   | MINUS
                   | STAR
                   | SLASH

<!-- <val> ::= literalint -->
```

The syntactic grammar specified above via BNF is a subset of the C language,
which will allow us to motivate the transition from recursive descent to pratt
parsing. It only recognizes and parses programs that return arithmetic expressions.
Note that the grammar's ordering is speecified according to precedence. You're
going to recognize `<program>` before an `expr`, right?

The `<program>` production refers to `<function>`, which, in turn, refers to
`<statement>`, but these can be easily rewritten as one single RE. The one
production which REs could not specify is the `<exp>` production; it refers to
itself an arbitary amount of times.

**Recursive Descent: Specification**

A recursive descent parser is what we call parsers that parse tokens the same way
you lex characters into tokens. They have three synonyms:
- *recursive descent*: because they *descend* down the grammar's spec
- *top-down* parsers: because they start from the *top* and go *down*

Recursive descent/top down parsers are *predictive* parsers because they recognize
and produce the correct production based on, usually, a single character of lookahead
without any backtracking

Parsers which implement recursive descent lend well to a hand-written implementation,
since you have a single function per non-terminal. Whether you are recursing with
the host language's stack frames or an explicit stack data structure, you just
have to make sure to avoid the non-halting scenario from incorrect base/inductive
case ordering.

**Recursive Descent: Implementation**
```
<expr> ::= <expr> <binop> <expr>
         | LITERAL_INT
```

*Problem 1: Left recursion: does not halt*
Given a simple BNF grammar for a calculator (just expressions) like the one above,
the first line is problematic if we directly translate it with a recursive
implementation.

The naive workaround (at leaast to me) is to rearrange the order of `<expr>`'s
rules to recognize `LITERAL_INT` tokens first, before arbitrary expressions.
But if the parser was implemented that way, how does it know it captured the
entire expression? That was the entire purpose of ordering the grammar's
productions according to precedence.

```rust
fn parse_expr(tokens: &[Token]) -> Expr {
  let e = parse_expr() // there ain't no base case ==> ∞ does not halt ∞
  let op = parse_binop()
}

```

As [matklad explains](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing),
academia points out that left-recursive grammars are the the Achilles heel of
recursive descent grammars, which, theoretically, motivates LR parsing techniques.
Practically, you can stick with LL parsing pattern. Just use a loop.

```rust
fn parse_expr(tokens: &[Token]) -> Expr {
  match tokens {
    [] => panic!(),
    [f, r @ ..] => match f.typ {
      TokenType::LiteralInt => {
        // 1. create root with left initialized
        let mut root = if let Ok((op, _)) = parse_binop(r) {
            Expr::Binary {
                op,
                l: Box::new(Expr::Num(f.lexeme.parse().unwrap())),
                r: Box::new(Expr::Num(-1)),
            }
        } else {
            Expr::Num(f.lexeme.parse().unwrap())
        };

        // 2. initialize &mut root, and r_tokens, continually updated by loop
        let mut cur_node = &mut root;
        let mut r_tokens = r;

        // 3. while there still exists ops in input
        //    fill in right childs
        while let Ok((_, r)) = parse_binop(r) {
          // check: last loop ==> construct Expr::Num, not Expr::Binary
        }

        // 4. return
        Ok((root, r_tokens))
      },
      _ => panic!()
    }
  }
}
```

This implementation now halts and seems to produce the correct answer for
expressions like:
- `9 + 10 + 11`
- `9 - 10 - 11`
- `9 * 10 * 11`
- `9 / 10 / 11`

*Problem 2: Precedence*

The parser now halts for all inputs, but does it produce the correct answer for
all inputs? Yeahh...no; it does not handle precedence *across* operators well.

- `9 * 10 + 11`
- `9 / 10 - 11`

Society has decided as convention that we're going to perform multiplications
and divisions before additions and subtractions. This is as arbitrary as
what side of the road a country drives on, or what endianness an ISA interprets
machine words.

The parser above recognizes and produces right-heavy parse tree by default. The
parse tree's semantics do not match society's agreed upon semantics for arithmetic.
If we evaluate the parse tree via software interpretation, or generate assembly
for hardware interpretation, we're going to get incorrect answers.

```
                                 *
                                / \
  9 * 10 + 11 -> |parser| ->   9   +
                                  / \
                                 10 11

                                 /
                                / \
  9 / 10 - 11 -> |parser| ->   9   -
                                  / \
                                 10 11
```

The solution for this is to stratify the exp production even further according to
precedence levels.

```
<term>   ::= <factor>
           | <term> (PLUS|MINUS) <factor>
<factor> ::= <atom>
           | <term> (STAR|SLASH) <atom>
<atom>   ::= LITERAL_INT
           | PUNC_LEFT_PAREN <expr> PUNC_RIGHT_PAREN
```

The key for intuition is translate  `<term>` and `<factor>` into their
implementations. They're going to recognize and parse the left sub-tree which
has a stronger precedence level than the current level of recursion. `parse_term`
is going to recur on `parse_factor` first, and `parse_factor` will recur on
`parse_atom` first for their left sub-trees. Then, they will loop

step through an execution trace of a top-down parser
with a single character lookahead, given input 9 * 10 + 11.

The intuitive structure of implementation gets completely lost with this grammar.
This grammar is ok for parser generator, but the nature of it goes against the
whole point of hand writing your parser.

TODO: the solution below (implementing a stratified grammar across term/factor)
fixes associative solution too.
- just rock with for a bit, and update it later.

```rust

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    parse_term(tokens)
}

fn parse_term(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_factor(tokens)?;
    println!("moose {:?}", left);

    match r {
        [] => Ok((left, r)),
        r => {
            let mut root = left;
            let mut r_tokens = r;

            while let Ok((op, r)) = parse_term_op(r_tokens) {
                let (right, r) = parse_factor(r)?;

                root = Expr::Binary {
                    op,
                    l: Box::new(root),
                    r: Box::new(right),
                };

                r_tokens = r;
            }

            Ok((root, r_tokens))
        }
    }
}

fn parse_term_op(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Plus => Ok((Op::Add, r)),
            TokenType::Minus => Ok((Op::Sub, r)),
            _ => {
                // println!("{:?}", f);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        },
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_atom(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut root = left;
            let mut r_tokens = r;

            while let Ok((op, r)) = parse_factor_op(r_tokens) {
                let (right, r) = parse_atom(r)?;

                root = Expr::Binary {
                    op,
                    l: Box::new(root),
                    r: Box::new(right),
                };
                println!("wolf {:?}", root);

                r_tokens = r;
            }

            Ok((root, r_tokens))
        }
    }
}

fn parse_factor_op(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Star => Ok((Op::Mult, r)),
            TokenType::Slash => Ok((Op::Div, r)),
            _ => {
                // println!("{:?}", f);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        },
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => Ok((Expr::Num(f.lexeme.parse().unwrap()), r)),
    }
}
```


*Problem 3: Associativity*

`Parser halts with wrong answer: (does not bind correctly *within* operators)`

Test Case 1: Addition (Pass)
```
                                 +
                                / \
  9 + 10 + 11 -> |parser| ->   9   +
                                  / \
                                 10 11
```
Multiplication works for the same reasons.

Test Case 2: Subtraction (Fail)
```
                                 -
                                / \
  9 - 10 - 11 -> |parser| ->   9   -
                                  / \
                                 10 11
```
Division fails for the same reasons.



Whether or not the compiler
considers this to be a legal

They key here is to remember the order of recursion.

Sol: ???


*Problem 2: precedence*
- sol 1: lookahead? messy for hand written
- sol 2: fix grammar? structure gets hidden
- sol 3: pratt parsing

// problem 1: precedence
(+ (* 9 10) 3)
      +
     / \
    *   3        =   93
   / \
  9  10


9 * 10 + 3;

        *
       / \
      9   +       =   117
         / \
        10  3

// sol 1: ❌
maybe...
((9 * 10) + 3);
((9 * (3 + 6)) + 10)


// how do we find op. we have to look ahead from paren
// that's why s expressions are easy, op is with opening paren (lookahead > 1)
// which then implies tables, b/c by hand is messy to lookahead more than 1: why?

// sol 2: fix grammar ✅

// problem 2: associativity
// -> how do you make a RD parser not left recursive AND non left associative??

**Pratt Parsing**

[Pratt Parsing References: ](https://www.oilshell.org/blog/2017/03/31.html)


### D. Semantic Analysis: Type Checking

Lexical analysis detects lexical errors (ill-formed tokens), syntactic analysis
detects syntax errors, and semantic analysis detects semantic errors, such as
static type errors, undefined variables, and uninitialized variables.

Once semantic analysis is complete and successful, the program must be a legal
program in the programming language; no further errors in the program should be
reported by the compiler.


### E. Source Language References (C89/90)
- [C Standards (Drafts)](https://github.com/sys-research/c-standard-drafts)
- The C Programming Language (K&R)
- If You Must Learn C (Ragde)

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

# 3. Backend (Generation)
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

- reference card (green, homage to IBM 360)

### RV32I (Stable target)

### RV32G (Standard extensions)

- mult and divide: RV32M
- floating point: RV32F and RV32D
- atomic: RV32A

## E. Target Language 2: LLVM
- [LLVM for Grad Students (Sampson)](https://www.cs.cornell.edu/~asampson/blog/llvm.html)
- [Greenplace (Bendersky)](https://eli.thegreenplace.net/tag/llvm-clang)
- [Compilers and IRs (Zhang)](https://www.lei.chat/posts/compilers-and-irs-llvm-ir-spirv-and-mlir/)
- [AOSA: LLVM (Lattner)](https://aosabook.org/en/v1/llvm.html)
- [Tourist's Guide to the LLVM Source Code (Regehr)](https://blog.regehr.org/archives/1453)