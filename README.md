# Rust for ML and Software Engineers

A hands-on Rust learning repository tailored for practical engineering use. This repository walks through Rust fundamentals to advanced concepts with runnable examples, focusing on the needs of ML Engineers, Data/Platform/Infra engineers, and Software Engineers building reliable, high-performance systems.

**Dedication**: This repository is dedicated to the Colab ML Engineering community and to ML/Software Engineers all over the world.

---

## What you will find here

- **Step-by-step learning path**: Each numbered folder introduces a concept with small, runnable Rust programs.
- **Executable examples**: Most sections are Cargo projects with multiple `bin` examples. Run them directly.
- **Engineering-first framing**: Examples are designed to build intuition for correctness, performance, and safety.
- **Jupyter-friendly setup**: Optional notebooks and setup guides for experimenting with Rust (via tools like `evcxr` or system Rust).

---

## Repository structure

The repository is organized as a progression of topics. Each topic usually contains:
- a `project/` or topic-named Cargo workspace with a `src/bin/` folder of examples
- a top-level `README.md` for that topic (when applicable)
- occasionally auxiliary files (samples, data, etc.)

Highlights:
- `00_Setup_and_Reading/` — Installer and Jupyter setup notes; starter notebooks
- `01_Hello_World/` — First Rust programs with and without Cargo
- `02_Variables_and_Mutability/` — Variables, mutability, shadowing, constants, scopes
- `03_Data_Type/` — Scalars, compound types, conversions
- `04_Functions/` — Function definitions, parameters, returns
- `05_Control_Flow/` — `if`, `loop`, `while`, `for`, pattern-based flow
- `06_Ownership/` — Ownership, moves, clones, borrowing
- `07_Slices/` — String and array slices
- `08_Structs/` — Structs, methods, associated functions
- `09_Enums/` — Enums and pattern matching
- `10_Generics/` — Generic types and functions
- `11_Option_and_Result_Enums/` — Idiomatic error handling primitives
- `12_Vectors/` — Dynamic arrays, iteration patterns
- `13_Strings/` — UTF-8 strings, operations, pitfalls
- `14_HashMaps/` — Key-value storage, ownership nuances
- `15_Error_Handling/` — `Result`, `?`, custom errors
- `16_Traits/` — Trait bounds, default methods, object safety
- `17_Lifetimes/` — Borrow checker mental model, lifetime annotations
- `18_Closures/` — Captures, `Fn` traits, practical usage
- `19_Iterators/` — Iterator adapters, lazy evaluation, performance
- `20_Testing_in_Development/` — Unit, integration tests; dev workflow

> Each topic directory contains its own Cargo project you can run. Many topics also include multiple binaries under `src/bin/` for focused examples.

---

## Prerequisites

- Rust toolchain (stable) with Cargo
  - Install via `rustup` (recommended): https://www.rust-lang.org/tools/install
- Linux, macOS, or Windows (WSL recommended on Windows)
- Optional: Jupyter + `evcxr_jupyter` for Rust notebooks if you want to use notebooks

---

## Why Rust for ML and Engineering?

- **Performance without garbage collection**: Predictable latencies for data and systems work.
- **Memory and thread safety**: Compiler-enforced guarantees reduce entire classes of bugs.
- **Great for productionization**: When models/algorithms need to ship as reliable services, libraries, or embedded components.
- **Ecosystem**: Growing crates for data processing, numerical computing, serving, and tooling.

---

## Attribution

This repository draws substantial inspiration from the Udemy course “Learn to Code with Rust.” Roughly 80% of the examples are adapted from or based on materials covered there. All credit for the original curriculum and teaching belongs to the course creator. This project is community-maintained and not affiliated with or endorsed by the instructor or Udemy.

Support the author: [Course Link](https://www.udemy.com/share/10cbdb3@9L0E3vbcETIkgLogGfcI5loc0Z6monuOsNCkX6Hz3Uckd2a2AEJlQnPhAwhJqtJyYw==/).