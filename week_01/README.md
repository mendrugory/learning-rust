# learning-rust
Palmtree 'learning Rust' Week 1

## Assignment 1: Install Rust
```bash
$ ./assignment_01/install_ubuntu.sh
```

## Assignment 2: Hello World
```bash
$ rustc -o assignment_02/hello assignment_02/hello.rs
$ ./assignment_02/hello
```

## Extras
### Hello World using Cargo
```bash
$ cd extras && cargo new hello_world --bin
$ cd hello_world
$ cargo run
```

### Hello Name using Cargo
```bash
$ cd extras && cargo new hello_name --bin
$ cd hello_name
```
```rust
use std::env;

fn main() {
    if let Some(name) = env::args().nth(1){
        println!("Hello {}!", name);
    }
}
```
```bash
$ cargo run Pepe
Hello Pepe !
```