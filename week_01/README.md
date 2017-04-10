# learning-rust
Palmtree 'learning Rust' Week 1

## Assignment 1: 
* Install Rust
```bash
$ ./assignment_01/install_ubuntu.sh
```

* Hello World
```bash
$ rustc -o assignment_01/hello assignment_02/hello.rs
$ ./assignment_01/hello
```


## Assignment 2: 
```bash
$ cd assignment_02/fibonnaci
$ cargo run
```

## Assignment 3: 
```bash
$ cd assignment_03/guessing
$ cargo run
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

Modify src/main.rs
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