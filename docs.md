# catnukh_matrix_parser Documentation

## Purpose
The goal of this project is to provide an educational example of a parser for matrix operations in Rust.  
It demonstrates how to define a grammar for matematical operations for matrices, implement a parser using **pest**, and build an interpreter that executes matrix operations such as addition, subtraction, multiplication, and scaling.

## About
The project consists of:
- **lib.rs** — the library containing data structures (`Matrix`, `Command`), parsing logic, and error handling.
- **main.rs** — the CLI interface that accepts a file with commands, invokes the parser, and runs the interpreter.
- **matrix.pest** — the formal grammar describing the syntax of matrix operations.
- **tests/** — unit tests that validate parsing correctness for each grammar rule.

# To Setup the project
```rust
   git clone
```
To start: 
```
cargo --help
```

## Pipeline
1. **Input**: a text file containing commands (e.g., `mat A = [[1,2],[3,4]]`).
2. **Parsing**: the pest grammar parses the file into a syntax tree.
3. **Command Conversion**: the syntax tree is converted into a list of `Command` objects.
4. **Execution**: the interpreter processes the commands, stores matrices in memory, and performs the requested operations.
5. **Output**: results of the operations are printed to the console.

## Grammar
Key grammar rules (`matrix.pest`):

```pest
WHITESPACE = _{ " " | "\n" | "\t" | "\r" }
COMMENT    = _{ "#" ~ (!"\n" ~ ANY)* }
number     = { "-"? ~ ('0'..'9')+ ~ ("." ~ ('0'..'9')+)? }
name_of_matrix = { (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
row        = { "[" ~ number ~ ("," ~ number)* ~ "]" }
matrix     = { "[" ~ row ~ ("," ~ row)* ~ "]" }
mat_def    = { "mat" ~ WHITESPACE* ~ name_of_matrix ~ WHITESPACE* ~ "=" ~ WHITESPACE* ~ matrix }
add        = { "add" ~ WHITESPACE* ~ name_of_matrix ~ WHITESPACE* ~ "," ~ WHITESPACE* ~ name_of_matrix }
subtract   = { "sub" ~ WHITESPACE* ~ name_of_matrix ~ WHITESPACE* ~ "," ~ WHITESPACE* ~ name_of_matrix }
mult       = { "mul" ~ WHITESPACE* ~ name_of_matrix ~ WHITESPACE* ~ "," ~ WHITESPACE* ~ name_of_matrix }
scale      = { "scale" ~ WHITESPACE* ~ name_of_matrix ~ WHITESPACE* ~ "," ~ WHITESPACE* ~ number }
operation  = { mat_def | add | subtract | mult | scale }
file       = { SOI ~ operation* ~ EOI }