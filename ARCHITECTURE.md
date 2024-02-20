# System Architecture
This is a high level document covering system architecture. Refer to
[COMPILER-ARCHITECTURE](./COMPILER-ARCHITECTURE.md) and [CHIP-ARCHITECTURE](./CHIP-ARCHITECTURE.md)
for more information on compilers and chips.

**Contents**
1. What is computing?
2. System Software (Compilers)
3. Hardware (Chips)

## What is computing?
The purpose of computing is insight, not numbers[^0]. And humans compute by
orchestrating electrons. However, there's semantic chasm between us, and our
electron cousins. So this communication mismatch is bridged by many different
layers of abstraction.

This document assumes you're familiar with application programming. It goes over
the principles of system design[^1] used to implement a machine which the end
programmer programs.

```
---------------------
| Problem           |
---------------------
| Algorithm         |
---------------------
| Language, Runtime |
---------------------
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
re-evaluated, and machines are being specifically design for machine learning
workloads. Din (this project) and [nayru]() explore the design and implementation
of general-purpose and parallel[^4] compute machines, while [farore]() covers
machine-learning machines.

As stated previously, system design at the machine level is not so different
from system design at the distributed level. The goal of the system is to bridge
the semantic gap between programmers and electrons, and like any large problem,
it's broken down into many subproblems (hence, the layers of abstraction).

However, it's important to realize that any piece of the system, whether it be
language, runtime, or microarchitecture can implement a capabiltiy...
- i'm kind of stretching here. this is where it's helpful to know what's at the bottom
  (link to feynman). b/c then you know what's possible with gates! polynomial instructions?
- hardware is essentially an interpreter. difference between interpeter and compiler
- tradeoff, etc. hence hardware/software codesign
- while the focus is on compiler implemtnation (i might build a RISC simulator. we'll see.)
- food for thought: probsting's law (link)

- semantic gap
- becomes more obvious when talking about parallel software models and hw implementations


## System Software (Compilers)
1. maintain source semantics, translate to target semantics
2. optimize on some metric


- compiler at a high level
- different IRs

## Hardware (Chips)
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