# Grim
Project Grim, a small interpreted and imperative language.

# Syntax
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
To print a variable or an expression: <br>
```
let a = "test";
print (a);
print ("Example");
```

## Type management
To view a precise management of types compatibility see expression_evaluator.rs. <br>
In general the only casting that can happen is int -> float if, for instance, an int is summed with a float. <br>

## Next steps
Future features:
- [ ] Function implementation
- [ ] Fancier error messages
