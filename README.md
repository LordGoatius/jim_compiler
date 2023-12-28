# Interpreter (Functioning)

The REPL can be ran by simply executing the program

> $ cargo run

Alteratively, a file can be passed and it will be executed by the Interpreter

> $ cargo run -- file.jim

# Compiler

Compiled executables can be generated by passing a file name as the input, and including the -C flag

> $ cargo run -- -C file.jim

(Currently generates a nonfunctional assembly file)
