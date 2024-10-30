# Grim
Grim, a small interpreted and imperative language.

# Syntax
## Identifiers
An identifier can start with [a-z_] and then can contain any character in [a-zA-Z0-9_].

## Possible statements
Below a list of supported statements with their syntax.
### Variable declaration
A variable can be declared using ```let``` keyword. <br>
A variable name must be declared with [a-zA-Z], after it, it can contain any alphanumeric character. <br>
A declared variable must be initialized, the type is currently inferred. <br>
Some examples:
```
let a = 0;
let b = 120.0;
let c = true;
let d = c && !c;
let e = "Test";
```
> [!TIP]
> Grim is locally scoped. <br>
> A block is defined by:
> - ```if```/```if-else```,
> - ```while```
> - ```function call```
> 
> A block can see every variable declared in the father blocks, father blocks cannot see locally declared variables. <br>
> Variables overshadowing is currently not supported.

### Variable assignment
A value is assigned simply with ```=```. <br>
An example:
```
let a = 0;
a = 1;
```

### If-If/else
If and if-else are supported. <br>
An example:
```
let a = true;
let b = false;
if a || b {
    b = true;
}
if a && !b {
    b = false;
} else {
    let c = 0.0;
}
```

### While
While is supported. <br>
An example:
```
let a = true;
let b = false;
while a || b {
    b = true;
}
```

### Print statement
To print a variable, an expression or a string: <br>
```
let a = "test";
print (a);
print ("Example");
```
If you want to go to a new line you can use `printl`.

### Input statement
To input interactively a value in a variable:
```
let a = 0;
input(a);
```

### Function declaration and call
A function is declared with the following syntax:
```
fn fun_name (arg1, arg2) -> { return arg1 + arg2; }
```
A function must end with an explicit return. <br>
Calling a function without assigning the return value means that the value is discarded after call. <br>
A function can be called simply by:
```
let a = 0;
let b = 1;
let c = fun_name(a, b);
fun_name(a, b);
```
Pass is only by value.

## Type management
To view a precise management of types compatibility see expression_evaluator.rs. <br>
In general the only casting that can happen is int -> float if, for instance, an int is summed with a float. <br>

# Run the language
You need Rust and Cargo installed. <br>
First you will have to build the project with:
```
cargo build
```
To run, while in the terminal:
```
cargo run -- path_to_grim_script
```

# Customize the language
You can add features to the language:
1. Define the tokens into `lexer.rs`,
2. Define the grammar rules using tokens into `grammar.lalrpop`
3. Define the interpreter rules into `interpreter.rs`

Eventually you may want to add more arithmetical/logic operators, to do so you have to edit `expression_evaluator.rs`.

# Next steps
Future features:
- [ ] Lambda functions
- [ ] REPL
- [ ] Fancier error messages
