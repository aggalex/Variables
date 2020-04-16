# Variables
A simple example of parsing and AST building with Rust and LALRPOP

### What can it do?
It can successfuly parse and execute expression evaluation, variable creation and evaluation and printing. It also has comments.
An example of what the program can do is in the `examples/simple` file

### Syntax
- `variable_name = <expression>`
- `print <expression>`

An expression is a simple series of mathematical of mathematical operations between constants or variables, just like in most languages. They can be evaluated to a single value. For now, all values are integers in this example.

- Operations
  - `val1 + val2`: Addition
  - `val1 - val2`: Subtraction
  - `val1 * val2`: Multiplication
  - `val1 / val2`: Division
  
`val1` and `val2` are either initialized variables or immidiate constants, like `1`, `2`, `3`, `4`...

### Building and running
To build the project you will need the Rust building system `cargo`. Most Linux distributions have `cargo` in their repositories, from which you can install the recommended version of cargo by your distribution maintainer. If however you are not using Linux or wish to install the version recomended by the `cargo` developers, you can follow these instructions: https://doc.rust-lang.org/cargo/getting-started/installation.html

to build the project, go the root folder of the project and run

	cargo build

then again, if you wish to run the interpreter, use

	cargo run <file>

replacing <file> with the file containing the code you want the interpreter run.
