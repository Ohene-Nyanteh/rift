# Rift

> A simple scripting language created by me cuz i can

Rift is a lightweight, interpreted scripting language written in Rust. It's minimal by design — no bloat, no nonsense.

---

## Features

### Done
- Variables (`let`)
- Arithmetic & logical operators (`+`, `-`, `*`, `/`, `%`, `and`, `or`, `xor`, `nor`)
- Comparison operators (`==`, `!=`, `>`, `<`, `>=`, `<=`)
- Control flow (`if`, `elif`, `else`)
- Loops (`while`)

- Comments (`#`)
- Print

### Planned
- `for` loops
- `match` statements
- Functions (Don't ask me why i havent done this one)
- Return, break, continue
- Structs
- Module system
- Standard library

---

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) installed on your machine

### Installation

```bash
git clone https://github.com/Ohene-Nyanteh/rift.git
cd rift
cargo build --release
```

### Running a script

```bash
./target/release/rift index.rf
```

---

## Example

```
# FizzBuzz in Rift
let i = 1;
while (i <= 20) {
    if (i % 15 == 0) {
        print("FizzBuzz");
    } elif (i % 3 == 0) {
        print("Fizz");
    } elif (i % 5 == 0) {
        print("Buzz");
    } else {
        print(i);
    }
    i = i + 1;
}
```

---

## Version

`v0.0.1` — Turing complete. The rest is just features (I just dont want to do the rest. lol).

---

## License

MIT

---
