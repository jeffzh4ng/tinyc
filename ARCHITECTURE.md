# System Architecture
This is a high level document covering system architecture. For more information
specific to compilers and chips, refer to
[COMPILER-ARCHITECTURE](./COMPILER-ARCHITECTURE.md) and [CHIP-ARCHITECTURE](./CHIP-ARCHITECTURE.md)

**Contents**
1. What is computing?
2. System Software (Compilers)
3. Hardware (Chips)

## What is computing?
The purpose of computing is insight, not numbers[^0]. And humans compute by
orchestrating electrons. However, there's a semantic chasm that lies between us,
and our fellow electron cousins. So this communication mismatch is bridged with
many different layers of abstraction.

This document assumes you're familiar with application programming. It goes over
the *principles* of system design[^1] used to implement a machine which the end
programmer programs.

```
--------------------- <-- Web app, mobile app
| Problem           |
--------------------- <-- Sequences, Trees, Graphs
| Algorithm         |
--------------------- <-- C/C++/Java/Python
| Language, Runtime |
--------------------- <-- x86/ARM/RISC-V
| Microarchitecture |
---------------------
| Gates             |
---------------------
| Electrons         |
---------------------
```

An understanding of principles is important, as it allows you to design machines
for different problems. For the past fifty years, machines were designed for
general-purpose computing, largely enabled by smaller, cheaper, and more energy
efficient transistors[^2].

Today, those underlying enablers are slowing down[^3], while others (namely,
machine learning) are speeding up. As a result, a lot of assumptions are being
re-evaluated, and machines are now being specifically design for machine learning
workloads. Din (this project) and [nayru]() explore the design and implementation
of general-purpose and parallel[^4] compute machines, while [farore]() covers
machine-learning machines.

As stated previously, system design at the machine level is not so different
from system design at the distributed level. The goal of the system is to bridge
the semantic gap between programmers and electrons, and like any large problem,
it's broken down into many subproblems (hence, the layers of abstraction).

However, to understand design *principles* rather than *precedent* is to ask
why the semantic chasm has been divided like so. Why do we need a system software
layer providing programmer's high level languages and runtimes? Don't real[^5]
programs use computers out of drums and vacuum tubes[^6] afterall? Fundamentally,
what are the assumptions that underlie the current design (solution)? For some
workloads[^7], perhaps it makes sense to implement kernel bypass, or, remove the
kernel all together.

## System Software (Language, Runtime)
Any craftsman realizes different tools serve different purposes. There's a time
and place for speed-first languages (C/C++), and for safety-first languages
(Java/Python)[^8]. There's also a time and place for single-machine runtimes
such as Unix and Linux, as well as multi-machine ones like Kubernetes, and Temporal.

Focusing our attention now on languages, for the end programmer to achieve
insight, they're going to need an answer[^9]. That answer is always going
to be produced by an interpreter, and that interpreter can be implemented with
system software (with another high level language, called the host language) or
hardware (with gates and electrons).

Whether the interpreter is implemented with system software or[^10] hardware is an
implementation detail. A languages' interface is better thought of as its syntax
and semantics[^11]. So then, if interpreters produce answers, why do we need compilers?

We need compilers when we're implementing an interpreter with hardware. That's
because a hardware interpreter's (a processor, really) implements a simpler
language, often times called assembly or machine languages. Such as x86 or ARM.

So,
- if you're writing a program in C, C++, Java or Python
- and it's not going to be interpreted by system software
- then it has to be interpeted by hardware
- which only speaks assembly/machine languages such as x86, ARM and RISC-V
- so thus we need a compiler to translate between the two
- just like din, which translates C to RISC-V.

## Hardware (Microarchitecture)
Picking up where we just started, we label the assembly/machine languages such
as x86 or ARM as Instruction Set Architectures (ISA).




- becomes more obvious when talking about parallel software models and hw implementations


1. maintain source semantics, translate to target semantics
2. optimize on some metric


1. maintain source semantics, translate to target semantics

system software (compilers and operating systems) and
hardware
(microarchitecture) which act in unison in order to provide an execution model
for the programmer.

 in order to provide an execution model for the programmer,
which in turn implements algorithms to solve problems for the end user.

[^0]: Richard Hamming
[^1]: not in the distributed sense, but rather, single-machine
[^2]: see Moore's Law and Dennard Scaling
[^3]: the apocryphal type like to declare "the end of Moore's Law"
[^4]: parallel compute is still fundamentally general (von Neumann execution model)
[^5]: see https://paulgraham.com/avg.html
[^6]: see https://users.cs.utah.edu/~elb/folklore/mel.html
[^7]: databases, high frequency trading
[^8]: we even have Rust now, which solves for both with it's static permission system
[^9]: errors too, and in the worst case, infinite loops
[^10]: JIT compilation blurs the exclusive or
[^11]: according to Harper, the only real (formally specified) language is SML