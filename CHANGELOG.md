# 0.1.0

- Added instructions : 
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

# 0.2.0

- Improved error messages

# 0.3.0

- Added instructions :
  - tee \<register_a> \<register_b> : Test if register_a == register_b
  - tne \<register_a> \<register_b> : Test if register_a != register_b
  - tll \<register_a> \<register_b> : Test if register_a < register_b
  - tmm \<register_a> \<register_b> : Test if register_a > register_b
  - tel \<register_a> \<register_b> : Test if register_a <= register_b 
  - tem \<register_a> \<register_b> : Test if register_a >= register_b
  - jmp \<instruction> : Jump to \<instruction> if Eq register is true

# 0.3.1

- Added instructions :
  - inc \<register> : Increments the register
  - dec \<register> : Decrements the register

# 0.3.2

- Fixed inverted Dec & Inc
- Added prt instruction to print ascii char corresponding to letter value

# 0.3.3

- Added instruction dmp

# 0.3.4

- Added dump command

# 0.3.5

- Removed instructions :
  - set
  - peek
  - inc
  - dec

- Added better error messages

# 0.3.6

- Correcting stack bug making stack starting at 1

# 0.3.7

- Being the same as jump while moving to ip (-3)

# 0.3.8

- Added exception for popping the last element of the stack

# 0.3.9

- Fixed jump problem (bad decrementation)

# 0.3.10

- Removed useless intstruction \`Dst`