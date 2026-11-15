# Rift

> A simple scripting language I made because I can. (Its a hobby Project)

Rift is a lightweight, interpreted scripting language written in Rust. Minimal by design — no bloat, no nonsense. File extension: `.rf`

---

## Installation

**Prerequisites:** [Rust](https://rustup.rs/) installed on your machine.

```bash
git clone https://github.com/Ohene-Nyanteh/rift.git
cd rift
cargo build --release
```

**Running a script:**

```bash
./target/release/rift index.rf
```

---

## Command-line Usage

```bash
rift <file> [flags]
```

| Flag        | Description                                      |
|-------------|--------------------------------------------------|
| `--help`    | Show the help menu and exit.                     |
| `--version` | Show the Rift version and exit.                  |
| `--upgrade` | Update Rift if a newer release is available.     |

### Updater

Rift has a built-in updater. If Rift was installed from an official release or installer, run:

```bash
rift --upgrade
```

It checks for a newer release, installs it when available, and reports the result:

```
Updated to 0.1.3
```

or, when already current:

```
Rift is already up to date
```

---

## Language Reference

### Comments

Comments start with `#` and run to the end of the line.

```
# this is a comment
let x = 10; # inline comment
```

---

### Variables

Variables are declared with `let`. They are dynamically typed — you can assign any value to any variable.

```
let name = "Ohene";
let age = 20;
let score = 9.5;
let active = true;
```

Reassigning a variable does not need `let` again:

```
let count = 0;
count = count + 1;
```

---

### Data Types

Rift supports the following value types:

| Type    | Example              |
|---------|----------------------|
| Integer | `42`                 |
| Float   | `3.14`               |
| String  | `"hello"`            |
| Boolean | `true`, `false`      |
| Array   | `[1, 2, 3]`          |

---

### Arrays

Arrays are declared with square brackets and are dynamic — they can hold any mix of types and grow at runtime.

```
let fruits = ["apple", "banana", "mango"];
let nums = [1, 2, 3, 4, 5];
```

Access elements by index (zero-based):

```
let first = fruits[0];  # "apple"
let third = nums[2];    # 3
```

You can also assign to an index:

```
fruits[1] = "pear";
```

---

### Operators

**Arithmetic:**

```
let a = 10 + 3;   # 13
let b = 10 - 3;   # 7
let c = 10 * 3;   # 30
let d = 10 / 3;   # 3.33...
let e = 10 % 3;   # 1
```

**Comparison:**

```
10 == 10   # true
10 != 5    # true
10 > 5     # true
10 < 5     # false
10 >= 10   # true
10 <= 9    # false
```

**Logical:**

```
true & false   # false
true | false   # true
!true           # false
```

---

### Print

Use `print` to output a value to the console.

```
print("Hello, world!");
print(42);
print(1 + 2);
```

---

### Conditionals

Use `if`, `elif`, and `else` for branching logic. Conditions go in parentheses.

```
let score = 75;

if (score >= 90) {
    print("A");
} elif (score >= 75) {
    print("B");
} elif (score >= 60) {
    print("C");
} else {
    print("F");
}
```

---

### Functions

Functions are declared with `fn`. They can take any number of arguments.

```
fn greet(name) {
    print("Hello, " + name);
}

greet("Ohene");
```

Functions can return values with `return`:

```
fn add(a, b) {
    return a + b;
}

let result = add(3, 7);
print(result);  # 10
```

---

### While Loop

Repeats a block as long as a condition is true.

```
let i = 0;

while (i < 5) {
    print(i);
    i = i + 1;
}
```

---

### Loop (Count-up from a number)

`loop number from N` runs a block indefinitely, starting a counter at `N`. Useful for counting loops without a fixed end condition.

```
loop count from 0 {
    print(count);

    if (count == 9) {
        break;
    }
}
```

The counter variable (`count` above) is available inside the block and increments on each iteration.

---

### Flow Control

`return`, `break`, and `continue` control execution flow.

`return` exits a function and optionally passes back a value:

```
fn firstPositive(nums) {
    let i = 0;
    while (i < 5) {
        if (nums[i] > 0) {
            return nums[i];
        }
        i = i + 1;
    }
}
```

`break` exits a loop early:

```
let i = 0;
while (i < 100) {
    if (i == 5) {
        break;
    }
    i = i + 1;
}
```

`continue` skips the rest of the current iteration and moves to the next:

```
let i = 0;
while (i < 10) {
    i = i + 1;
    if (i % 2 == 0) {
        continue;
    }
    print(i);  # prints only odd numbers
}
```

---

### Enums

Enums define a type with a fixed set of named variants. Declare with `enum`, then reference variants using `::`.

```
enum Direction {
    North,
    South,
    East,
    West
}

let heading = Direction::North;
```

Enum variants have no sub-values — they are simple named constants belonging to their type.

---

### Structs

Structs group related fields together under a single name. Fields are declared with default values and accessed using dot notation.

```
struct Point {
    x: 0,
    y: 0,
    z: 0
}

let p = Point;
p.x = 10;
p.y = 20;

print(p.x);  # 10
print(p.z);  # 0
```

---

### Match

`match` compares a value against a series of cases and runs the first one that matches. Use `default` to handle any value not explicitly listed. Cases always use curly braces.

```
let status = 2;

match status {
    1 => { print("one"); }
    2 => { print("two"); }
    default => { print("something else"); }
}
```

Match works with any value type — integers, strings, booleans, enum variants, variables, and structs.

```
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter
}

let current = Season::Winter;

match current {
    Season::Spring => { print("Flowers blooming"); }
    Season::Summer => { print("Hot outside"); }
    Season::Autumn => { print("Leaves falling"); }
    Season::Winter => { print("Cold and dark"); }
    default => { print("Unknown season"); }
}
```

---

### Modules, Imports, and Exports

A module is just another `.rf` file. Use `@export` to share a declaration, and `@import` to use it from another file.

**math.rf**

```
@export fn add(a, b) {
    return a + b;
}
```

**main.rf**

```
@import { add } from "./math.rf";

print(add(2, 3));  # 5
```

`@export` can be placed before `let`, `fn`, `struct`, and `enum`:

```
@export let app_name = "Rift";

@export enum Color {
    Red,
    Green,
    Blue
}

@export struct Point {
    x: 0,
    y: 0
}
```

Import paths are strings and are relative to the file doing the importing:

```
@import { add } from "./math.rf";       # same folder
@import { helper } from "../lib/util.rf";  # parent folder
```

Before execution, Rift walks the imports and builds a module graph. Each file is loaded once and run **bottom-up**: dependencies first, then the file that imported them.

```
main.rf  ──imports──>  math.rf

Execution order:  math.rf  →  main.rf
```

All modules share one global environment, so a name declared at the top level of an imported module is available after that module runs.

#### Circular imports cannot happen

Rift refuses to run circular imports. During the import walk it keeps the active chain of files on a stack. If a file tries to import something already on that stack, Rift stops with a `Circular import` error.

```
a.rf  ──imports──>  b.rf  ──imports──>  c.rf
  ^                                       │
  └────────────────imports────────────────┘

Circular import:
 a.rf -> b.rf -> c.rf -> a.rf
```

The check happens **before any code runs**, so a cycle can never produce an infinite load loop. The same file imported by two different files is fine — it is loaded only once.

```
      main.rf
      /     \
     v       v
  left.rf  right.rf
      \     /
       v   v
      shared.rf      # loaded once, executed once
```

---

## Full Example

FizzBuzz in Rift:

```
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

## Planned Features

- Standard library

---

## Version

`v0.1.3` — Turing complete, with modules and a built-in updater.

---

## License

MIT
