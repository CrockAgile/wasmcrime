# Wasm Spec Notes

## Overview

[(reference)][overview]

### Values

* 32 and 64 bit width
* 32 bit integers also serve as Booleans **and memory addresses**
* no storage distinction between signed and unsigned. instead **operands interpret sign specifics**

there is also a single 128 bit wide vector type

values can also be **opaque references**, pointers toward different types of entities

references cannot have their size or representation observed

### Architecture

* stack machine
* control flow is structured

### Traps

some instructions can produce a **trap**, which immediately aborts execution

can *not* be handled by wasm, but only the executing environment

### Functions

* sequence of parameters
* sequence of return values
* can declare mutable local variables

### Tables

* array of opaque values of a particular element type
* select elements via dynamic index operand
* currently **only untyped function reference or external host value**
* pretty much function pointers

### Linear Memory

* created with initial size
* grown dynamically
* load/store at any byte address **including unaligned**
* integer load/store can be smaller size (truncates?)
* **trap** if access not in current memory bounds

### Modules

* definitions for functions, tables, linear memories, global variables
* definitions **can be imported and exported**
* can initialize data, via segments copied to given offsets
* can define **start** that is auto executed

### Embedder

Host environment gotta run all this!

### Semantic Phases

* decode
* validate
* instantiate
* invoke

## Structure

[(reference)][structure]

### Number Types

`numtype ::= i32 | i64 | f32 | f64`

`vectype ::= v128`

### Reference Types

References, either function or external, reference objects in the runtime "store"

`reftype ::= funcref | externref`

function references are the *infinite* union of all references to functions

external references are the *infinite* union of all objects owned by the "embedder" runtime

reference types are *opaque*, meaning neither size nor bit pattern can be observed

reference types can be stored in "tables"

### Value Types

`valtype ::= numtype | vectype | reftype`

### Result Types

`resulttype ::= [vec(valtype)]`

### Function Types

`functype ::= resulttype -> resulttype`

### Global Types

`globaltype ::= (const | var) valtype`

### External Types

`externtype ::= func functype | table tabletype | mem memtype | global globaltype`

## Instructions

[(reference)][instructions]

* clz, ctz, popcnt
* add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr
* abs, neg, sqrt, ceil, floor, trunc, nearest
* add, sub, mul, div, min, max, copysign
* eqz, eq, ne, lt, gt, le, ge
* const, extend, promote, wrap, reinterpret

also a lot of SIMD vector instructions

* const, shuffle, sqizzle, splat, ...

### References

null, is_null, or get function reference

### Parametric

`drop` throws away a single operand

`select` gets one of its first two operands bases on if third is zero

### Variable

local get,set,tee

global get,set

### Table

get, set, grow, fill, copy, init, drop

### Memory

load, store, size, grow, fill, copy, init, drop

### Control

`blocktype ::= typeidx | valtype`

* `nop` does nothing
* `unreachable` causes a trap
* `block`, `loop`, `if` structure nested sequences of instructions called "blocks"

label 0 is the "innermost" control instruction, increasing outward

branches can only be directed outward

`block` and `if` jump *forward* to the current control instruction `end`
`loop` jumps *backward* to the current control instruction's start

* `br` unconditional branch
* `br_if` conditional branch
* `br_table` indirect branch via parameter index table

branches can consume arguments, which are pushed back onto stack after unwinding

* `call` invokes another function, consuming stack arguments
* `call_indirect` invokes another function via parameter index table

because the `funcref` table can have varying function types, the callee is dynamically checked against the indexed function type, and may trap

## Execution

[(reference)][execution]

### Values

* `num ::= [i32|i64|f32|f64].const`
* `vec ::= v128.const`
* `ref ::= ref.null | ref funcaddr | ref.extern externaddr`
* `val ::= num | vec | ref`

each value type defaults to 0 or null

*results* are either values or a trap

### Store

all global state manipulated by wasm

### Addresses

indexes into the "store" list

addresses are **not** indices

* address is dynamic, globally unique reference to runtime objects
* indices are static, module-local references to their definition

## Binary Encoding

* integers are encoded using LEB128, variable length
* floats are IEEE 754
* names are UTF8
* types are byte constants
* tables are limits & reference type (func or extern)
* globals are their value type, and a flag for mutability
* instructions are byte constants
* block instructions are structured, with both starting and ending bytes (e.g. `block instr* end`)
* `section = section id | byte length | contents`
* `module = magic (0asm) | version (1000) | sections*`

[overview]: https://webassembly.github.io/spec/core/intro/overview.html
[structure]: https://webassembly.github.io/spec/core/syntax/index.html
[instructions]: https://webassembly.github.io/spec/core/syntax/instructions.html
[execution]: https://webassembly.github.io/spec/core/exec/runtime.html
