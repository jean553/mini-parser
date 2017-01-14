# simple-parser

Attempt to create a top-down recursive descent parser for a C-like language.

## Objectives

Parse an input, generate x86 assembly code according to this input.
Parsing is made according to a C-like grammar (very basic one).
The grammar is written during the parser development.

My goal is to discover how parsing process works and 
learn more about compilation.

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
