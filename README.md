[![Build Status](https://travis-ci.org/jean553/mini-parser.svg?branch=master)](https://travis-ci.org/jean553/mini-parser)

# mini-parser

Attempt to create a top-down recursive descent parser for a C-like language.

## Tasks in progress

 * add details about Lex and Yacc files and code
 * add details about BNF into the documentation

## Objectives

Parse an input, generate x86 assembly code according to this input.
Parsing is made according to a C-like grammar (very basic one).
The grammar is written during the parser development.

My goal is to discover how parsing process works and 
learn more about compilation.

## Table of content

 - [Current supported syntax](#current-supported-syntax)
 - [Lex and Yacc](#lex-and-yacc)
    * [Lex](#lex)
    * [Yacc](#yacc)
    * [Parser generation steps](#parser-generation-steps)
    * [Files sections](#files-sections)
 - [Working examples](#working-examples)
    * [Basic operation](#basic-operation)
    * [Program generation with nasm](#program-generation-with-nasm)
 - [Build the environment](#build-the-environment)
 - [Compile](#compile)
 - [Execute](#execute)
 - [Documentation](#documentation)
 - [Compilation details](#compilation-details)
    * [exit syscall](#exit-syscall)
    * [Entrypoint](#entrypoint)

## Current supported syntax

```bnf
<sum_operation> ::= <digit> '+' <digit>
<digit>         ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

## Lex and Yacc

The grammar can be verified using Lex and Yacc.
The .lex and .y files can be found in lex_yacc/ folder.

### Lex

Lex is the "Lexical Analyser". It gets an input and find the "interesting bits" from this input,
like `(`, `{`, `[`, `;`... it also locate individual characters and their words.

For example, the lexical analyser gets the following input:

```
int value = 5 + 2;
```

And generates the following tokens:

```
int
value
=
5
+
2
;
```

Usually, it produces a list of the smallest tokens the given grammar can support.

### Yacc

Yacc is a parser, and means "Yet Another Compiler Compiler".
Yacc triggers actions (write program output, create syntax trees...)
when items of the grammar are recognized after the lexical analysis.

Yacc **generates C code** that "parses" the given grammar.

This C code can then be compiled in order to generate a parser
that is able to understand our grammar.

This is useful to check if the developed parser (here in Rust)
actually does what is expected by comparing the output
of code both given to the Yacc generated C-parser and the custom parser.

Build syntax tree:

```
        =
  +---+   +---+
  |           |
  v           v
value
              +
        +---+   +---+
        |           |
        v           v

        5           2

```

Generated code (pseudo-assembly):

```asm
mov ax, 5
add 2
mov [value], ax
```

Lex and Yacc are complimentary tools.

### Parser generation steps

The following steps are required to generate a Parser using Lex and Yacc.

Generate the syntax analyzer (parser). It uses the grammar description from `test.y`.
(-d option in order to expose tokens as defines into the generated .h file)

```
yacc -d test.y
```

Generate the lexical analyser. It uses definition from `test.lex`
and tokens from the `yacc` generated header file (`y.tab.h`).

```
lex test.lex
```

Generate the parser using a standard C compiler:

```
cc lex.yy.c y.tab.c
```

### Files sections

Lex and Yacc files have three sections (separated by `%%`):
 * header/control information,
 * token/grammar rules and definitions,
 * C-code to be copy/pasted into the final output 

#### Lex file sections

We examine here the sections of the `test.lex` file.

 * Before `%%`: control section, C code to be copied into the final output,
 * After the first `%%`: tokens section, regular expressions (tokens) with their respective actions
 * After the second `%%`: code to be copied into the final output
(unlike the first section, this section does not require any `%{` and `%}` symbols).

When parsing the input, everytime one of the regular expression is met,
the associated action is executed.

## Working examples

### Basic operation

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

### Program generation with nasm

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

### exit syscall

The `exit` syscall is automatically appended to the assembly code at the end.
It asks the kernel to stop the calling program.

```asm
mov eax, 1
int 0x80
```

The interrupt 0x80 calls the syscall `exit` if `eax` value is 1.

### Entrypoint

`_start` is the entrypoint, declared as global to be accessed by the system
in order to start the program execution.

This is the name searched by `ld` during linking process.
