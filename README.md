# wlvm

Virtual machine with associated language written in Rust

![Build & Test](https://github.com/Wafelack/wlvm/workflows/Build%20&%20Test/badge.svg)

## Installation

`cargo install wlvm`

## Build

```
git clone https://github.com/wafelack/wlvm
cd wlvm/
cargo build
cargo test
```

## Usage

### Run program

`wlvm run $program`

### Dump program's memory and registers

`wlvm dump $program`

## Details

<details>
<summary>Virtual Machine datasheet</summary>

## Registers

There are 6 multi purposes registers, marked from a to f.

There are 4 special registers : 
- sp : The stack pointer
- ip : The instruction pointer
- st : The stack top value
- eq : The result of the last test performed

## Instruction Set

### Stack instructions

- psh \<integer> : Pushes an integer onto the stack
- pop : Pops the stack

### Arithmetic operations

- add \<register_a> \<register_b> : Adds the content of register_b to register_a
- sub \<register_a> \<register_b> : Substracts the content of register_b to register_a
- mul \<register_a> \<register_b> : Multiplies the content of register_b to register_a
- div \<register_a> \<register_b> : Divides the content of register_a by register_b

### Memory operations

- mov \<register_a> \<register_b> : Copies content of register_b in register_a
- jmp \<instruction> : Jump to \<instruction> if Eq register is true

### Boole algebra operations

- tee \<register_a> \<register_b> : Test if register_a == register_b
- tne \<register_a> \<register_b> : Test if register_a != register_b
- tll \<register_a> \<register_b> : Test if register_a < register_b
- tmm \<register_a> \<register_b> : Test if register_a > register_b
- tel \<register_a> \<register_b> : Test if register_a <= register_b 
- tem \<register_a> \<register_b> : Test if register_a >= register_b

### Other

- dmp : Dumps the stack and the registers
- drg \<register> : Prints the content of the specified register
- prt \<register> : Prints the character corresponding to register value

- hlt : Stops the program




</details>

