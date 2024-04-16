#!/bin/bash

assert() {
  input="$1"
  expected="$2"

  ./target/release/din "$input" > tmp.s || exit
  riscv64-unknown-elf-gcc -o tmp tmp.s
  spike pk tmp
  actual="$?"

  if [ "$expected" = "$actual" ]; then
    echo "$input => $actual"
  else
    echo "$input => expected $expected, but got $actual"
    exit 1
  fi
}

# arithmetic
assert "./tests/fixtures/din/legal/arithmetic/lit.c" 8
assert "./tests/fixtures/din/legal/arithmetic/add.c" 19
assert "./tests/fixtures/din/legal/arithmetic/add_multi.c" 30
assert "./tests/fixtures/din/legal/arithmetic/sub.c" 56
assert "./tests/fixtures/din/legal/arithmetic/mult.c" 90
assert "./tests/fixtures/din/legal/arithmetic/div.c" 11

assert "./tests/fixtures/din/legal/arithmetic_precedence/add_associative.c" 30
assert "./tests/fixtures/din/legal/arithmetic_precedence/sub_associative.c" 11
assert "./tests/fixtures/din/legal/arithmetic_precedence/mult_add_precedence.c" 101
assert "./tests/fixtures/din/legal/arithmetic_precedence/mult_add_precedence_multi.c" 222

# control flow
assert "./tests/fixtures/din/legal/control_flow/lt.c" 1
assert "./tests/fixtures/din/legal/control_flow/lteq.c" 1
assert "./tests/fixtures/din/legal/control_flow/lteq2.c" 1
assert "./tests/fixtures/din/legal/control_flow/gt.c" 1
assert "./tests/fixtures/din/legal/control_flow/gteq.c" 1
assert "./tests/fixtures/din/legal/control_flow/gteq2.c" 1


echo OK