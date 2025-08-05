# Hello, World! in Rust — Using `rustc` and Cargo

This guide shows how to create and run a **Hello, World!** program in Rust using two approaches:

- **Manual** — `rustc` (Rust compiler only)
- **Recommended** — Cargo project (Rust’s build tool and package manager)

## **1. Hello, World! with `rustc` (Manual)**

This method uses only the **Rust compiler**, no Cargo.

### 1 Create the source file

You can create the file manually or via terminal:

```bash
touch hello_world.rs
````

Now write this in `hello_world.rs`:

```rust
// hello_world.rs
fn main() {
    println!("Hello, World!");
}
```

### 2 Compile with `rustc`

```bash
rustc hello_world.rs
```

This compiles the code and creates an executable called:

* `hello_world` (on Linux/macOS)
* `hello_world.exe` (on Windows)

### 3 Run the executable

```bash
./hello_world
```

### Quick Reference (Manual)

| Task        | Command                |
| ----------- | ---------------------- |
| Create file | `touch hello_world.rs` |
| Compile     | `rustc hello_world.rs` |
| Run         | `./hello_world`        |

---

## **2. Hello, World! with Cargo (Recommended)**

> **Project name:** `hello_world_cargo`

Cargo is the **recommended approach** for real-world Rust projects because it handles project structure, dependencies, testing, and more.

### 1 Create a new Cargo project

```bash
cargo new hello_world_cargo
```

This creates the following project structure:

```
hello_world_cargo/
├── Cargo.toml
├── src/
│   └── main.rs
└── target/
    ├── debug/
    │   ├── build/
    │   ├── deps/
    │   ├── examples/
    │   ├── incremental/
    │   ├── hello_world_cargo
    │   ├── hello_world_cargo.d
    │   └── .fingerprint/
    ├── release/
    └── CACHEDIR.TAG
```

### 2 Open `src/main.rs`

It contains:

```rust
fn main() {
    println!("Hello, World!");
}
```

### 3 Build and run the project

```bash
cd hello_world_cargo
cargo run
```

### Quick Reference (Cargo)

| Task           | Command                       |
| -------------- | ----------------------------- |
| Create project | `cargo new hello_world_cargo` |
| Build + Run    | `cargo run`                   |
| Check code     | `cargo check`                 |
| Format code    | `cargo fmt`                   |


## **What’s inside the `target/` folder?**

| Subfolder / File      | Purpose                                              |
| --------------------- | ---------------------------------------------------- |
| `debug/`              | Default output of `cargo build` and `cargo run`.     |
| `release/`            | Output of `cargo build --release` (optimized build). |
| `debug/deps/`         | Compiled dependencies.                               |
| `debug/incremental/`  | Incremental compilation data.                        |
| `debug/.fingerprint/` | Build fingerprints for cache invalidation.           |
| `hello_world_cargo`   | Compiled binary executable.                          |
| `hello_world_cargo.d` | Dependency info.                                     |
| `CACHEDIR.TAG`        | Indicates cache directory (optional).                |

You **don’t need to manually touch `target/`** — Cargo manages everything automatically.

---

## **Why use Cargo?**

While `rustc` is fine for **small scripts**,
Cargo is essential for:

* Managing **multiple source files**
* Handling **dependencies** (external crates)
* Running **tests**
* Generating **documentation**
* Building optimized **release** builds