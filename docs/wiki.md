# 📚 Kode Programming Language Wiki

> A comprehensive guide to the Kode programming language

**Created by Sreeraj V Rajesh**

---

## 📋 Table of Contents

- [Introduction](#-introduction)
- [Installation](#-installation)
- [Language Version](#-language-version)
- [Running Kode Programs](#-running-kode-programs)
- [Language Syntax](#-language-syntax)
- [Standard Library](#-standard-library)
- [Examples](#-examples)
- [Language Comparisons](#-language-comparisons)
- [Current Limitations](#-current-limitations)
- [Future Development](#-future-development)

---

## 🌟 Introduction

Kode is a modern, interpreted programming language designed with simplicity and readability in mind. It features a C-like syntax with influences from JavaScript, Python, and Rust, making it easy to pick up for developers familiar with these languages.

The language supports essential programming concepts like variables, functions, arrays, closures, control flow structures, and a basic module system. It's implemented in Rust, making it memory-safe and relatively fast for an interpreted language.

---

## 🛠️ Installation

To use Kode, you need to compile the Rust source code:

```bash
# Clone the repository
git clone https://github.com/cyberkutti-iedc/kode
cd kode

# Build with Cargo
cargo build --release

# Optional: Add to your PATH
echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
kode version
```

---

## 📊 Language Version

**Current Version: 0.2.0**

Check your installed version with:

```bash
kode version
```

---

## 🚀 Running Kode Programs

### Command Line Interface

Kode offers several command line options for running and building programs:

```bash
# Run a Kode program
kode run myprogram.kode

# Compile a Kode program to bytecode
kode build myprogram.kode

# Run compiled bytecode
kode run myprogram.kdc

# Start REPL mode
kode repl

# Display version information
kode version

# Show help
kode help
```

#### Command Line Options

- `--verbose`: Enable verbose output
- `--optimize`: Enable optimization (for build command)
- `--time`: Show execution time
- `--no-run`: Build only, don't run (for build command)

Examples:
```bash
kode run myprogram.kode --verbose
kode build myprogram.kode --optimize --no-run
kode run myprogram.kode --time
```

### REPL Mode

The Read-Eval-Print Loop (REPL) allows for interactive programming:

```bash
kode repl
```

```
Kode Programming Language v0.2.0 REPL
Type 'exit' or press Ctrl+C to quit
Type 'help' for available commands
> let x = 5;
> let y = 10;
> print x + y;
15
> exit
```

REPL Commands:
- `help`: Show available commands
- `exit`: Exit the REPL
- `clear`: Clear the screen

---

## 📝 Language Syntax

### Comments

```kode
// This is a single-line comment

/* This is a 
   multi-line comment */

/* Comments can be /* nested */ if needed */
```

### Variables

```kode
let name = "John";
let age = 30;
let pi = 3.14;
let isActive = true;

// Reassignment
age = 31;
```

### Data Types

- **Integer**: `let x = 42;`
- **Float**: `let pi = 3.14;`
- **Boolean**: `let isReady = true;`
- **String**: `let name = "Alice";`
- **Array**: `let numbers = [1, 2, 3, 4, 5];`
- **Function/Closure**: `let add = fn(a, b) { return a + b; };`

### Operators

#### Arithmetic Operators
- `+`: Addition
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division
- `%`: Modulo (remainder)

#### Comparison Operators
- `==`: Equal to
- `!=`: Not equal to
- `<`: Less than
- `>`: Greater than
- `<=`: Less than or equal to
- `>=`: Greater than or equal to

#### Logical Operators
- `&&`: Logical AND
- `||`: Logical OR
- `!`: Logical NOT

### Control Flow

#### If-Else Statements
```kode
if (condition) {
    // code to execute if condition is true
} else if (anotherCondition) {
    // code to execute if anotherCondition is true
} else {
    // code to execute if all conditions are false
}
```

#### While Loops
```kode
while (condition) {
    // code to execute while condition is true
}
```

#### For Loops
```kode
for (let i = 0; i < 5; i = i + 1) {
    print i;
}
```

### Functions

```kode
fn add(a, b) {
    return a + b;
}

// Main function (entry point)
fn main() {
    let result = add(5, 3);
    print result;  // 8
}
```

### Arrays

```kode
let fruits = ["apple", "banana", "cherry"];
print fruits[0];  // "apple"
fruits[1] = "blueberry";  // Modify element
```

### Closures

```kode
let add = fn(a, b) {
    return a + b;
};

let result = add(5, 3);  // 8
```

### Error Handling

```kode
try {
    // code that might cause an error
    riskyOperation();
} catch {
    // error handling code
    print "An error occurred";
}
```

### Modules and Imports

```kode
// In math.kode
fn square(x) {
    return x * x;
}

// In main.kode
import math;
print math.square(5);  // 25
```

---

## 📦 Standard Library

The Kode standard library provides built-in functions for common operations:

| Function | Description |
|----------|-------------|
| `print(value)` | Outputs a value to the console |
| `len(array)` | Returns the length of an array |
| `type(value)` | Returns the type of a value as a string |
| `parse_int(string)` | Converts a string to an integer |
| `parse_float(string)` | Converts a string to a float |
| `to_string(value)` | Converts a value to its string representation |

*Note: The standard library is still under development and more functions will be added in future versions.*

---

## 📝 Examples

### Hello World

```kode
fn main() {
    print "Hello, World!";
}
```

### Factorial

```kode
fn factorial(n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    print factorial(5);  // 120
}
```

### Array Manipulation

```kode
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    // Sum all numbers
    let sum = 0;
    for (let i = 0; i < 5; i = i + 1) {
        sum = sum + numbers[i];
    }
    
    print sum;  // 15
    
    // Double each number
    for (let i = 0; i < 5; i = i + 1) {
        numbers[i] = numbers[i] * 2;
    }
    
    print numbers;  // [2, 4, 6, 8, 10]
}
```

### Error Handling

```kode
fn divide(a, b) {
    if (b == 0) {
        // This would trigger an error
        return a / b;
    }
    return a / b;
}

fn main() {
    try {
        print divide(10, 0);
    } catch {
        print "Cannot divide by zero";
    }
}
```

---

## 🔄 Language Comparisons

### Kode vs JavaScript
- Similar syntax for variables, functions, and control flow
- Kode is simpler with fewer built-in objects and methods
- Kode lacks JavaScript's prototype-based OOP and more advanced features

### Kode vs Python
- Kode uses explicit braces for blocks instead of indentation
- Kode requires semicolons to end statements
- Kode's syntax is closer to C-family languages
- Python has more extensive libraries and language features

### Kode vs Rust
- Kode is interpreted while Rust is compiled
- Kode is dynamically typed while Rust is statically typed
- Kode lacks Rust's ownership system and advanced type features
- Syntax has some similarities but Kode is much simpler

---

## ⚠️ Current Limitations

Kode is in active development and has several limitations:

1. **No Object-Oriented Programming** - No classes or methods
2. **Limited Type System** - Dynamic typing with no type annotations or checks
3. **Basic Standard Library** - Limited built-in functions and utilities
4. **Performance** - As an interpreted language, it's not as fast as compiled languages
5. **Basic Error Handling** - Simple try-catch with no specific error types
6. **Limited Module System** - Basic import functionality without namespacing
7. **No Async Support** - No built-in support for asynchronous programming
8. **Limited Collections** - Only arrays, no dictionaries/maps, sets, etc.

---

## 🔮 Future Development

See the [roadmap](roadmap.md) for detailed development plans. Key areas of focus include:

1. Adding more data structures (maps, sets)
2. Implementing a robust standard library
3. Adding object-oriented programming features
4. Improving error handling with specific error types
5. Adding type annotations and optional static type checking
6. Implementing a package manager
7. Adding async/await functionality
8. Improving performance with JIT compilation

---

*Wiki maintained by Sreeraj V Rajesh*

© 2025 Kode Programming Language