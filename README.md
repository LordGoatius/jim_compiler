# Interpreter (Functioning)

The REPL can be ran by simply executing the program

> $ cargo run

Alteratively, a file can be passed and it will be executed by the Interpreter

> $ cargo run -- file.jim

# Compiler

Compiled executables can be generated by passing a file name as the input, and including the -C flag

> $ cargo run -- -C file.jim

(Currently generates a nonfunctional assembly file)
# Syntax
Declare value: `<val> : 5 ;`

Print value: `<val> = ;`

Print result of math with value: `<operator> <val1> <val2> = ;`

Example
```
six : 6 ;
seven : 7 ;
eight : 8 ;
nine : 9 ;
* - + six seven eight nine = ;
+ nine 1 = ;
```
