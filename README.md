# Learning Rust

Walkthrough of https://doc.rust-lang.org/book

Up to: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html

## Install (Mac)

Install `rustup` with sudo and a C compiler (xcode-select):

```
sudo curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y --no-modify-path
xcode-select --install
```

Configure current shell:

```
source "$HOME/.cargo/env
```

Confirm rust is installed:

```
rustc --version
```

## Compile & run

```
cargo build
cargo run
```

Check without producing an executable:

```
cargo check
```

Build for release with optimisations (use this to benchmark programs):

```
cargo build --release
```

## View documentation for imported crates

```
cargo doc --open
```
