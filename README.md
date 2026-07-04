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

- Module system
- Standard library

---

## Version

`v0.0.1` — Turing complete. The rest is just features.

---

## License

MIT
