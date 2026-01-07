# ffi-typestate

This project demonstrates a **Rust ↔ C Foreign Function Interface (FFI)** using a
**typestate / ownership-split pattern** to enforce safe ownership transfer across
the language boundary.

The goal is to show how Rust can safely interact with C code while keeping
`unsafe` localized and preventing misuse by construction.

---

## What this project shows

- Direct Rust–C FFI using `extern "C"`
- Passing Rust-owned data to C via raw pointers
- C temporarily owning and mutating the data
- Rust regaining ownership safely
- Ownership enforced **at the type level**, not with runtime flags

---

## Core idea: ownership split via typestate

Ownership is represented explicitly using two Rust types:

```text
OwnedByRust  →  OwnedByC  →  OwnedByRust
