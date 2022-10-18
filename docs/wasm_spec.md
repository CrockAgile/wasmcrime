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

### Instructions

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

[overview]: https://webassembly.github.io/spec/core/intro/overview.html
[structure]: https://webassembly.github.io/spec/core/syntax/index.html
