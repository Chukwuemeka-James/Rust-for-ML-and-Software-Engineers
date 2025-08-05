# **Rust Installation Guide (Ubuntu Linux, macOS, Windows)**

This guide provides step-by-step instructions for installing the Rust programming language on Ubuntu Linux, macOS, and Windows. Rust empowers developers with a fast, safe systems language suitable for backend, systems, AI/ML infrastructure, and more.

## **Overview**

Rust installation is easy across all platforms via **rustup**, the official installer and version management tool for Rust.

---

## **1. Ubuntu Linux**

### Step 1: Install Dependencies

```bash
sudo apt update
sudo apt install build-essential curl
```

### Step 2: Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Press **1** to proceed with the default installation.

### Step 3: Configure your PATH

After installation, either restart the terminal or run:

```bash
source $HOME/.cargo/env
```

### Step 4: Verify Installation

```bash
rustc --version
```

---

## **2. macOS**

### Step 1: Install Xcode Command Line Tools (required)

```bash
xcode-select --install
```

### Step 2: Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Press **1** to proceed with the default installation.

### Step 3: Configure your PATH

Restart your terminal or run:

```bash
source $HOME/.cargo/env
```

### Step 4: Verify Installation

```bash
rustc --version
```

---

## **3. Windows**

### Step 1: Download rustup-init.exe

Visit: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Click **"Rustup-init.exe"** to download the installer.

### Step 2: Run the installer

1. Double-click **rustup-init.exe**.
2. Select **1** for default installation (recommended).

### Step 3: Verify Installation

Open **Command Prompt (cmd)** or **PowerShell**, then run:

```bash
rustc --version
```

---

## **🎥 Recommended Video Tutorials**

* **Rust Installation on Ubuntu** — [YouTube](https://youtu.be/DEy8Wg-eoZA?si=Exe3jR0gPUiqknym)
* **Rust Installation on Windows** — [YouTube](https://youtu.be/2PmPWWTmfiU?si=JtQSi7AE-B0JTiUT)
* **Rust Installation on macOS** — [YouTube](https://youtu.be/YL8TVC83mEs?si=EY30ZW_Dm1aT564u)

---

## **Post-installation (All Platforms)**

### 1. Update Rust (optional)

```bash
rustup update
```

### 2. Check Rustup version

```bash
rustup --version
```

### 3. Install Cargo (comes bundled with Rust)

Verify:

```bash
cargo --version
```

### 4. Uninstall Rust (optional)

```bash
rustup self uninstall
```

---

## **You’re all set!**

You now have Rust installed and ready to build powerful, fast, and safe applications.

Happy hacking with Rust!

> **This guide is part of a repository dedicated to Software Developers, Data Scientists, and AI/ML Engineers who want to learn Rust.**
