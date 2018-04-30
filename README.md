[![Build Status](https://travis-ci.org/jean553/mini-parser.svg?branch=master)](https://travis-ci.org/jean553/mini-parser)

# mini-parser

Attempt to create a top-down recursive descent parser for a C-like language.

## Tasks in progress

 * add details about Lex and Yacc into the documentation
 * add details about Lex and Yacc files and code
 * add details about BNF into the documentation

## Objectives

Parse an input, generate x86 assembly code according to this input.
Parsing is made according to a C-like grammar (very basic one).
The grammar is written during the parser development.

My goal is to discover how parsing process works and 
learn more about compilation.

## Current supported syntax (BNF)

```bnf
<sum_operation> ::= <digit> '+' <digit>
<digit>         ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

## Lex and Yacc

The grammar can be verified using Lex and Yacc.
The .lex and .y files can be found in lex_yacc/ folder.

### Lex

Lex is the "Lexical Analyser". It gets an input and find the "interesting bits" from this input,
like `(`, `{`, `[`, `;`... it also locate individual characters and their word.

## Working examples

#### Basic operation `x+y`:

Input:

```
4+6
```

Ouput:

```asm
bits 64

segment .text
    global _start

_start:
mov eax, 4
mov ebx, eax
mov eax, 6
add eax, ebx

mov eax, 1
int 0x80
```

#### Program generation with nasm

Write the code and parse it using mini-parser:

```sh
mini-parser 4+6
```

Use NASM Assembler to generate an object file:

```sh
nasm -f elf64 test.asm
```

In order to be linked using `ld`, the output object file
must be organized using a specific format.
We choose ELF 64 bits.

Use the linker to generate the final executable:

```sh
ld -o test test.o
```

The executable file can be run:

```sh
./test
```

## Build the environment

```sh
vagrant up
```

## Compile

```sh
cargo build --release
```

## Execute

```sh
./target/release/mini_parser {input}
```

## Documentation

Generate the documentation:

```sh
cargo rustdoc -- --no-defaults
```

## Compilation details

### exit() syscall

The `exit` syscall is automatically appended to the assembly code at the end.
It asks the kernel to stop the calling program.

```asm
mov eax, 1
int 0x80
```

The interrupt 0x80 calls the syscall `exit` if `eax` value is 1.

### Entry point

`_start` is the entrypoint, declared as global to be accessed by the system
in order to start the program execution.

This is the name searched by `ld` during linking process.
