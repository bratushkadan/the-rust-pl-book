# Chapter 1

## Compile & run program

```sh
rustc main.rs
```

```sh
./main
```

## Cargo

### New project

Create a new project:

```sh
cargo new hello_cargo
cd hello_cargo
```

### Run

```sh
cargo run (-q)
```

### Check code (without compiling)

```sh
cargo check 
```

### Build

Create debug build:

```sh
cargo build
```

Run debug build:

```sh
./target/debug/hello_cargo
```

### Build (release)

```sh
cargo build --release
```

Run release build:

```sh
./target/release/hello_cargo
```
