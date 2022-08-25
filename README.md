
# Ferrum

Ferrum is a toy language implemented in and inspired by Rust. It is statically typed,
data-driven and compiles down to an arbitrarily defined byte-code. Neither the language,
nor the byte-code interpreter is particularly well-designed, since this is just a side
project. Although, to improve execution speeds and to fiddle with the technology a bit,
I intend to implement JITer based on
[cranelift-jit](https://crates.io/crates/cranelift-jit)
or [LLVM](https://crates.io/crates/llvm-sys).

Parsing of the language is done through [peg](https://crates.io/crates/peg), which is an
excellent tool to quickly define a language parser in Rust. The Output of the parser is an
AST representation of the program, which is then used for code generation. There are more
efficient ways of doing this, but, again, this is just a toy language.

## Bytecode-Interpreter

The bytecode interpreter I use for this project is just a simple stack machine. I will
probably change it to a simple registry machine before implementing the JITer, though.

## Memory Safety

Since lifetime analysis is hella complicated, I will go the Java rout with every heap
pointer and use reference counters and a garbage collector to handle deallocation.
References to stack variables and consts will probably get limited life-time
analysis, since that should not be THAT hard to implement.

## Data Model

Data is grouped in structs. Raw data types and struct types can either be saved to the
local stack, or allocated on heap. Native Data types are:

- `i8` 8-bit signed integer
- `i16` 16-bit signed integer
- `i32` 32-bit signed integer
- `i64` 64-bit signed integer
- `i128` 128-bit signed integer
- `u8` 8-bit unsigned integer
- `u16` 16-bit unsigned integer
- `u32` 32-bit unsigned integer
- `u64` 64-bit unsigned integer
- `u128` 128-bit unsigned integer
- `f32` single precision float
- `f64` double precision float
- `char` 8-bit character
- `[T; len]` arrays
- `&T` references
- `*T` pointers
- `&[T]` slices

## Standard Lib

There will be a very basic `std::` namespace with basic functions and API bindings to
allow the code to interact with the world outside the VM.

## Sandboxing

Since the JITer may be turned off and API features may be disabled by the interpreter,
sandboxing may be a possibility. But, I am no security expert on these matters and since
almost all 'sandbox-ready' languages have had their fair share of security breaches in
the past, I would not recommend running mobile code in any security sensitive environment.

## Optimization

Thus far, the compiler does not do any optimisations to Ferrum source code. I may implement
basics, like common expression elimination, but this is not a promise, don't rely on it.
As far as the JITer is concerned, cranelift / LLVM probably does code optimisations to
some extent, but that is outside my control.

# Naming

I didn't want to spend too much time on the name of this language. If you happen to have
a legal claim on the 'Ferrum' name, feel free to contact me and I will gladly change it
to something else.

# License

This project is distributed under an MIT license. See 'LICENSE' in the root folder of
this project.

# Support

lmao