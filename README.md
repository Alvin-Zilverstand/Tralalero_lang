# Tralalero_lang: A Simple, Whimsical Programming Language

## Introduction

Tralalero_lang is a small, interpreted programming language designed for simplicity and a touch of whimsy. It's built with Rust and aims to provide a straightforward environment for learning basic programming concepts, experimenting with language design, or simply having fun. The language uses a unique set of keywords inspired by playful Italian phrases, making the coding experience a bit more lighthearted.

## Getting Started

To get started with Tralalero_lang, you'll need to have Rust installed on your system.

### 1. Install Rust

If you don't have Rust installed, you can install it using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions.

### 2. Clone the Repository

Clone the Tralalero_lang repository to your local machine:

```bash
git clone https://github.com/Alvin-Zilverstand/Tralalero_lang.git
cd Tralalero_lang
```

### 3. Build the Interpreter

Navigate to the `Tralalero_lang` directory and build the interpreter using Cargo:

```bash
cargo build
```

This will compile the `main.rs` file and create an executable in the `target/debug/` directory (e.g., `target/debug/Tralalero_lang.exe` on Windows, or `target/debug/Tralalero_lang` on Linux/macOS).

### 4. Run a Program

To run a Tralalero_lang program, execute the compiled interpreter followed by the path to your `.tralla` file:

```bash
./target/debug/Tralalero_lang.exe your_program.tralla
```
(On Windows, use `.\target\debug\Tralalero_lang.exe your_program.tralla`)

## Language Syntax and Semantics

Tralalero_lang is a line-by-line interpreted language. Each program must begin with `Tralalero Tralala` and end with `Bombardiro Crocodilo`.

### Program Structure

All Tralalero_lang programs must adhere to the following structure:

```tralla
Tralalero Tralala
// Your code goes here
Bombardiro Crocodilo
```

### Comments

You can add single-line comments using `//`:

```tralla
// This is a comment
let my_variable = 10; // This is also a comment
```

### Variables

Variables are declared and assigned using the `let` keyword. Tralalero_lang supports numbers (floating-point) and strings.

```tralla
let my_number = 123.45;
let my_string = "Hello, Tralalero!";
let another_var = my_number;
```

### Data Types

*   **Numbers:** Represented as floating-point numbers.
*   **Strings:** Enclosed in double quotes (`"`).

### Printing

Use the `Matteeeo` keyword to print values to the console. You can print string literals or the values of variables.

```tralla
Matteeeo "This will be printed.";
let greeting = "Ciao!";
Matteeeo greeting;
```

### Arithmetic Operations

Basic arithmetic operations (`+`, `-`, `*`, `/`) can be performed within `let` statements. The current implementation supports simple binary operations (operand operator operand).

```tralla
let num1 = 10;
let num2 = 5;
let sum = num1 + num2;       // sum will be 15
let difference = num1 - num2; // difference will be 5
let product = num1 * num2;   // product will be 50
let quotient = num1 / num2;  // quotient will be 2
```

### String Concatenation

The `Unire Corde` keyword is used to concatenate two strings (either literals or variables) and store the result in a new variable.

```tralla
let part1 = "Hello";
let part2 = "World";
Unire Corde full_string part1 part2; // full_string will be "HelloWorld"

let greeting_part = "Good";
Unire Corde final_greeting greeting_part "morning"; // final_greeting will be "Goodmorning"
```
**Note:** Due to current parsing limitations, avoid spaces within string literals when directly used with `Unire Corde`. It's recommended to assign strings with spaces to variables first, then use the variables for concatenation.

### Conditional Statements

Conditional logic is implemented using `Tung Tung Tung` (if) and `Ballerina Cappuccina` (else). Supported comparison operators are `==`, `!=`, `>`, `<`, `>=`, `<=`.

```tralla
let x = 10;
let y = 5;

Tung Tung Tung x > y
{
  Matteeeo "x is greater than y";
}
Ballerina Cappuccina
{
  Matteeeo "x is not greater than y";
}
```

### Loops

Fixed-iteration loops are supported using `Pinguino Arrabiato Fruti` followed by the number of iterations.

```tralla
Pinguino Arrabiato Fruti 3
{
  Matteeeo "This will print 3 times.";
}
```

### Functions

Functions are defined using `Lirili Larila` and called using `Trippi Troppi`. Functions can take arguments.

```tralla
Lirili Larila greet (name)
{
  Matteeeo "Hello, ";
  Matteeeo name;
}

Trippi Troppi greet("Alice"); // Calls the greet function with "Alice"
```

### Known Limitations

*   **Function Return Values:** Functions currently execute their body but do not return values that can be captured or used in the calling scope. Any `return` statements within a function are not processed to pass a value back.
*   **Complex Expressions:** The `let` keyword's arithmetic parsing is limited to simple `operand operator operand` structures. More complex mathematical expressions (e.g., `(a + b) * c`) are not supported.
*   **String Literals with Spaces in `Unire Corde`:** Directly using string literals containing spaces with `Unire Corde` can lead to parsing issues. Assign such strings to variables first.

## Examples

The `examples/` directory contains several `.tralla` files demonstrating various language features:

*   `calculator.tralla`: Basic arithmetic operations and variable usage.
*   `fibonacci.tralla`: Demonstrates loops and function calls (note the return value limitation).
*   `string_manipulation.tralla`: Shows string concatenation.

## Implementation Details (For Developers)

This section provides a brief overview of how the Tralalero_lang interpreter is structured and some of the design choices and challenges encountered during its development.

### Interpreter Core

The interpreter is implemented in `src/main.rs` as a single-pass, line-by-line parser and executor. It reads the `.tralla` file content, splits it into lines, and then processes each line sequentially.

### Parsing Strategy

*   **Line-by-Line Processing:** The interpreter reads and executes code one line at a time.
*   **Comment Stripping:** Before processing, each line is checked for `//` (single-line comments), and any content after `//` is removed.
*   **Whitespace Splitting:** Commands and arguments are primarily parsed by splitting the line by whitespace. This simplicity leads to some of the known limitations, particularly with string literals containing spaces.
*   **Semicolon Trimming:** Trailing semicolons (`;`) are trimmed from expressions and variable names during parsing to ensure correct value retrieval and storage.

### Variable Handling

Variables are stored in a `HashMap<String, String>`, where the key is the variable name and the value is its string representation. Numeric values are converted to strings before storage and parsed back to `f64` when used in arithmetic operations or comparisons.

### Arithmetic and Expression Evaluation

*   **`let` Keyword:** The `let` keyword handles both direct variable assignments (e.g., `let x = 10;`) and simple binary arithmetic expressions (e.g., `let sum = a + b;`).
*   **`get_value` Function:** This helper function attempts to parse a given string as an `f64` literal. If it fails, it checks if the string corresponds to a variable name and, if so, attempts to parse the variable's string value as an `f64`.
*   **Limited Expression Parsing:** The current arithmetic parsing within `let` is limited to `operand operator operand`. More complex expressions require a more sophisticated parsing algorithm (e.g., shunting-yard or abstract syntax tree generation), which is not yet implemented.

### Function Call Mechanism

*   **Function Definition (`Lirili Larila`):** Functions are stored with their name, argument list, and the start/end program counter (PC) lines of their body.
*   **Function Call (`Trippi Troppi`):** When a function is called, a new local scope (a cloned `HashMap` of variables) is created. Arguments are passed by value into this local scope. The interpreter then executes the lines within the function's body.
*   **No Return Value Propagation:** A significant current limitation is that functions do not return values to the calling scope. Any `return` statements within a function only terminate its execution; the value is not captured or made available to the caller. This means functions primarily serve for side effects (e.g., printing).

## Development

Tralalero_lang is an ongoing project. Contributions and suggestions are welcome!

## Future Enhancements

*   Implement function return values.
*   Add support for more complex mathematical expressions.
*   Introduce more data types (e.g., booleans, lists).
*   Improve error handling and reporting.
*   Add more built-in functions (e.g., string manipulation, type conversion).