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

# 1. expressions
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



# 2. control flow
# -- booleans
assert "./tests/fixtures/din/legal/control_flow/eq_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/eq_false.c" 0
assert "./tests/fixtures/din/legal/control_flow/neq_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/neq_false.c" 0

assert "./tests/fixtures/din/legal/control_flow/and_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/or_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/and_false.c" 0
assert "./tests/fixtures/din/legal/control_flow/or_false.c" 0

assert "./tests/fixtures/din/legal/control_flow/lt_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/lteq_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/lteq2_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/gt_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/gteq_true.c" 1
assert "./tests/fixtures/din/legal/control_flow/gteq2_true.c" 1

# -- conditionals
assert "./tests/fixtures/din/legal/control_flow/ifels_then.c" 0
assert "./tests/fixtures/din/legal/control_flow/ifels_els.c" 0

# -- loops

# -- functions



# 3. data transfer
assert "./tests/fixtures/din/legal/data_flow/asnmt.c" 8
assert "./tests/fixtures/din/legal/data_flow/asnmt_multi.c" 9
assert "./tests/fixtures/din/legal/data_flow/asnmt_multi_expr.c" 19
assert "./tests/fixtures/din/legal/data_flow/asnmt_multi_expr_var.c" 38


# -- bindings
# - introduction
# - update (++/+=, --/-=)
# - elimination
# - 1 moral judgement from plai: scope. spatial (static) > temporal (dynamic)

# -- malloc/free
# -- pointer/deref
# -- structs selec/deref
# -- alloc/dealloc fixed sized arrays



echo OK