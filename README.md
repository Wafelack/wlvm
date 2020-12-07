# wlvm

Virtual machine with associated language written in Rust

## Installation

`cargo install wlvm`

## Virtual Machine datasheet

### Registers

There are 6 multi purposes registers, marked from a to f.

There are 3 special registers : 
- sp : The stack pointer
- ip : The instruction pointer
- st : The stack top value

### Instruction Set

- psh \<integer> : Pushes an integer onto the stack
- add \<register_a> \<register_b> : Adds the content of register_b to register_a
- sub \<register_a> \<register_b> : Substracts the content of register_b to register_a
- mul \<register_a> \<register_b> : Multiplies the content of register_b to register_a
- div \<register_a> \<register_b> : Divides the content of register_a by register_b
- pop : Pops the stack
- set \<register> \<integer> : Sets register's value to the specified integer
- mov \<register_a> \<register_b> : Copies content of register_b in register_a
- dst : Prints the stack
- drg \<register> : Prints the content of the specified register
- peek : Prints the value of the st register
- hlt : Stops the program
- tee \<register_a> \<register_b> : Test if register_a == register_b
- tne \<register_a> \<register_b> : Test if register_a != register_b
- tll \<register_a> \<register_b> : Test if register_a < register_b
- tmm \<register_a> \<register_b> : Test if register_a > register_b
- tel \<register_a> \<register_b> : Test if register_a <= register_b 
- tem \<register_a> \<register_b> : Test if register_a >= register_b
- jmp \<instruction> : Jump to \<instruction> if Eq register is true