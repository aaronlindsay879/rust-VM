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
  - 0b00 -> Immediate (raw value)
  - 0b01 -> Direct (memory address)
  - 0b10 -> Register
- 24 bits for various operands

## Instructions
### Misc
| instruction | short description        | opcode (hex) | example  | meaning             |
|-------------|--------------------------|--------------|----------|---------------------|
| HLT         | halt                     | 00           | HLT      | Halts processing    |
| IGL         | illegal                  | 3F           | IGL      | Illegal instruction |

### Data transfer
| instruction | short description         | opcode (hex) | example    | meaning         |
|-------------|---------------------------|--------------|------------|-----------------|
| LDBI        | load byte immediate       | 01           | LDBI $1,0  | $1 <- 0         |
| LDBD        | load byte direct          | 01           | LDBD $1,0  | $1 <- MEM[0]    |
| LDHI        | load half-word immediate  | 02           | LDHI $1,0  | $1 <- 0         |
| LDHD        | load half-word direct     | 02           | LDHD $1,0  | $1 <- MEM[0..2] |
| LDWD        | load word direct          | 03           | LDWD $1,0  | $1 <- MEM[0..4] |
| STRBI       | store byte immediate      | 04           | STRBI $1,0 | MEM[0] <- $1    |
| STRHI       | store half-word immediate | 05           | STRHI $1,0 | MEM[0..2] <- $1 |
| STRWI       | store word immediate      | 06           | STRWI $1,0 | MEM[0..4] <- $1 |
| MOV         | move register             | 07           | MOV $0,$1  | $0 <- $1        |

### Arithmetic
| instruction | short description  | opcode (hex) | example       | meaning       |
|-------------|--------------------|--------------|---------------|---------------|
| ADDR        | add register       | 10           | ADDR $1,$2,$3 | $1 <- $2 + 3  |
| ADDI        | add immediate      | 10           | ADDI $0,10    | $0 <- $0 + 10 |
| SUBR        | subtract register  | 11           | SUBR $1,$2,$3 | $1 <- $2 - 3  |
| SUBI        | subtract immediate | 11           | SUBI $0,10    | $0 <- $0 - 10 |
| MULR        | multiply register  | 12           | MULR $1,$2,$3 | $1 <- $2 * 3  |
| MULI        | multiply immediate | 12           | MULI $0,10    | $0 <- $0 * 10 |
| DIVR        | divide register    | 13           | DIVR $1,$2,$3 | $1 <- $2 / 3  |
| DIVI        | divide immediate   | 13           | DIVI $0,10    | $0 <- $0 / 10 |

### Comparisons
All results are stored in special equality register

| instruction | short description            | opcode (hex) | example    | meaning  |
|-------------|------------------------------|--------------|------------|----------|
| EQI         | equal immediate              | 20           | EQI $0,10  | $0 == 10 |
| EQR         | equal register               | 20           | EQR $1,$2  | $1 == $2 |
| NEQI        | not equal immediate          | 21           | NEQI $0,10 | $0 != 10 |
| NEQR        | not equal register           | 21           | NEQR $1,$2 | $1 != $2 |
| GTI         | greater than immediate       | 22           | GTI $0,10  | $0 > 10  |
| GTR         | greater than register        | 22           | GTR $1,$2  | $1 > $2  |
| GTEI        | greater than equal immediate | 23           | GTEI $0,10 | $0 >= 10 |
| GTER        | greater than equal register  | 23           | GTER $1,$2 | $1 >= $2 |
| LTI         | less than immediate          | 24           | LTI $0,10  | $0 < 10  |
| LTR         | less than register           | 24           | LTR $1,$2  | $1 < $2  |
| LTEI        | less than equal immediate    | 25           | LTEI $0,10 | $0 <= 10 |
| LTER        | less than equal register     | 25           | LTER $1,$2 | $1 <= $2 |