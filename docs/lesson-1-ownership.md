# Rust

## Introduction

What is Rust?

- Systems programming language
- Performance similar to C/C++
- Safe memory management known at compile time (Borrow checker)
- Compile-time checks for concurrency issues
- Opinionated
- Good / standardized package manageemnt system
- Good / standardized tooling + build system
- Unit testing is easy and integrated
- Embedded code examples also run as part of tests (guaranteed to compile)
- Regular release cadence
- Hasn't pissed off the community yet with changes (Stable)

Why Not Rust?

- Library ecosystem still needs to mature. Might not have the same level of maturity as C++ existing packages.
- You might like debugging memory dumps by examining raw memory and the guesswork involved + luck
- Still requires mental overhead of memory management. Should you use this for a web server?
- Project written in a different language (C, C++) requires a lot of overhead for conversion. `unsafe` keyword might need to be used which defeats the purpose of Rust
- Slower compile times than C++ (without considering C++ linters)

## Ownership

### What is Ownership?

Idea: enforce RAII by default. Avoid shared state.

### Why is Ownership important?

Or alternatively.. why is shared state bad?

- With multiple readers/writers, it's unclear how they all interact with one another through side effects + atomicity of write operations. (race conditions/ concurrency issues)
- Who is responsible for cleaning up the memory? (memory leak)
- How do we know if memory is cleaned? (double free/ access after free)

## Ownership by Examples

1. Move semantics
1. References
1. How to Copy
1. Mutable references
1. Dangling references
1. Slices (or Views and why it matters)
    - `String` vs `&str`
    - `Vec` vs `&[i32]`
