# Directives 

| directive name      | implemented | action                                                                                                   |
|---------------------|-------------|----------------------------------------------------------------------------------------------------------|
| .align [n]          | &#9745;     | aligns the next data directive on a 2^n boundary, if not specified then all default alignment is 4 bytes |
| .ascii [string]     | &#9745;     | stores a non-null terminated string                                                                      |
| .asciiz [string]    | &#9745;     | stores a null terminated string                                                                          |
| .byte [b1, ..., bn] | &#9745;     | stores n bytes in successive locations                                                                   |
| .half [h1, ..., hn] | &#9745;     | stores n half-words (2 bytes) in successive locations                                                    |
| .word [w1, ..., wn] | &#9745;     | stores n words (4 bytes) in successive locations                                                         |
| .space [n]          | &#9745;     | leaves n bytes free                                                                                      |
| .data               | &#9745;     | marks the start of the data section                                                                      |
| .code               | &#9745;     | marks the start of the code section                                                                      |

# Assembly
## General comments
- 6 bit operand
- 2 bit addressing mode
  - 00 -> Immediate (raw value)
  - 01 -> Direct (memory address)
  - 02 -> Register
- 24 bits for various operands

## Instructions
### Misc
| instruction | short description        | opcode (hex) | example  | meaning             |
|-------------|--------------------------|--------------|----------|---------------------|
| HLT         | halt                     | 00           | HLT      | Halts processing    |
| IGL         | illegal                  | 3F           | IGL      | Illegal instruction |

### Data transfer
| instruction | short description        | opcode (hex) | example   | meaning         |
|-------------|--------------------------|--------------|-----------|-----------------|
| LBI         | load byte immediate      | 01           | LBI $1,0  | $1 <- 0         |
| LBD         | load byte direct         | 01           | LBD $1,0  | $1 <- MEM[0]    |
| LHI         | load half-word immediate | 02           | LHI $1,0  | $1 <- 0         |
| LHD         | load half-word direct    | 02           | LHD $1,0  | $1 <- MEM[0..2] |
| LWD         | load word direct         | 03           | LWD $1,0  | $1 <- MEM[0..4] |
| SBI         | store byte immediate     | 04           | SBI $1,0  | MEM[0] <- $1    |
| SHI         | store byte immediate     | 05           | SHI $1,0  | MEM[0..2] <- $1 |
| SWI         | store byte immediate     | 06           | SWI $1,0  | MEM[0..4] <- $1 |
| MOV         | move register            | 07           | MOV $0,$1 | $0 <- $1        |


