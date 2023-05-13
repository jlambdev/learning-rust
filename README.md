# Learning Rust

Walkthrough of https://doc.rust-lang.org/book

Up to: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number

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
