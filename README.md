[![Build Status](https://travis-ci.org/jean553/mini-parser.svg?branch=master)](https://travis-ci.org/jean553/mini-parser)

# simple-parser

Attempt to create a top-down recursive descent parser for a C-like language.

## Objectives

Parse an input, generate x86 assembly code according to this input.
Parsing is made according to a C-like grammar (very basic one).
The grammar is written during the parser development.

My goal is to discover how parsing process works and 
learn more about compilation.

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

```
mini-parser 4+6
```

Use NASM Assembler to generate an object file:

```
nasm -f elf64 test.asm
```

Use the linker to generate the final executable:

```
ld -s -o test test.o
```

The executable file can be run:

```
./test
```

## Compile

You must have Rust and Cargo installed.

```
cargo build
```

## Execute

Debug mode.

```
./target/debug/mini_parser {input}
```

## Documentation

Generate the documentation:

```
cargo rustdoc -- --no-defaults
```
