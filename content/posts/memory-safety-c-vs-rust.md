---
title: "Memory Safety: C vs Rust"
date: 2026-04-02
summary: "Why most C CVEs are memory safety bugs, and how Rust eliminates that class at the compiler level."
tags: [rust, c, security, memory-safety]
---

## The Problem with C

C gives you direct control over memory. That power is also the source of most security
vulnerabilities in C codebases. Buffer overflows, use-after-free, and null pointer dereferences
are not bugs introduced by careless programmers — they are the natural consequence of
manual memory management at scale.

When you allocate a buffer in C, you own that memory until you call `free()`. If you write
past the end of the buffer, the compiler lets you. If you access freed memory, the compiler
lets you. If you dereference a null pointer, the compiler lets you — and the program crashes,
or worse, behaves unpredictably in a way an attacker can exploit.

The canonical example is a stack buffer overflow. You allocate a fixed-size buffer on the
stack and read user input into it without checking the input length. An attacker who controls
the input can write past the buffer boundary and into the stack frame — overwriting the
return address, redirecting execution to attacker-controlled data.

This is not a hypothetical. It's been the mechanism behind some of the most significant
security vulnerabilities in software history.

## What Rust Does Differently

Rust enforces memory safety at compile time through its ownership system.
Every value has exactly one owner. When the owner goes out of scope, the memory is freed
automatically — no `malloc`/`free`, no garbage collector, no reference counting overhead.

The borrow checker extends this: you can have either one mutable reference or any number
of immutable references to a value at a time, but not both simultaneously.
This prevents entire classes of bugs. A use-after-free is impossible in safe Rust because
the compiler proves at compile time that no reference outlives the data it points to.
A data race in concurrent code — two threads writing to the same memory — is a compile
error, not a runtime crash.

Buffer overflows don't happen in safe Rust because slice indexing is always bounds-checked.
`buf[i]` panics cleanly at runtime if `i >= buf.len()`. It doesn't silently write into
adjacent memory.

The word "safe" is specific here. Rust has an `unsafe` keyword that lets you opt out of
these guarantees for specific code blocks — raw pointer arithmetic, FFI calls into C
libraries, low-level hardware access. The `unsafe` surface is small and explicit.
A code review can focus on those blocks. The rest of the codebase has the compiler's
guarantee.

## The CVE Class This Eliminates

Approximately 70% of CVEs in Microsoft's codebase and 70% of Chrome's security bugs are
memory safety issues. These figures have been cited by Microsoft's Security Response Center
and Google's Project Zero independently, arriving at the same number from different codebases.

The implication is stark: if those codebases had been written in a memory-safe language,
70% of the security bugs would not exist structurally. Not fixed — *structurally impossible*.

Rust's ownership model makes buffer overflows, use-after-free, and dangling pointer
dereferences impossible in safe mode. An attacker cannot exploit a vulnerability that
cannot exist.

This does not mean Rust code has no vulnerabilities. Logic errors, incorrect input
validation, cryptographic mistakes, and business logic flaws are all still possible.
The `unsafe` blocks in Rust FFI layers are still worth auditing carefully.
But the single largest CVE category in existing C and C++ codebases is simply removed
from the risk surface.

## What This Looks Like in Practice

The port scanner in GeistScope handles raw network connections. In C, a naive implementation
might read data from a socket into a fixed-size buffer and pass a raw pointer to that buffer
to a parsing function. If the parsing function doesn't validate the length, a crafted
response from a malicious host could overflow the buffer.

In Rust, the buffer is a `Vec<u8>` with a known length. The `read()` call returns how many
bytes were actually read. The slice passed to the parser is bounded. The compiler verifies
that the reference doesn't outlive the buffer it points into.

The developer doesn't have to remember to check the length. The type system enforces it.

That's the meaningful difference between C and Rust for security-sensitive code —
not that Rust programmers are more careful, but that the language makes a category of
mistakes structurally impossible before the code ever runs.
