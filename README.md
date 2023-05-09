Rust VM with bare-bones directive and instruction set. 
```asm
.data                       
        .align 2            ; align following directive to 2 bytes
string: .asciiz "a"         ; store the null-terminated string "a"
.code
loop:   prtsd @string       ; prints string stored at label string
        ldbd $0, @string    ; loads the raw value of label string (so the location in memory) into register 0
        addi $0, 1          ; adds 1 to register 0
        gti $0, 0x7a        ; check if the new value in register 0 is greater than 0x7a (the character 'z')
        jmpei @end          ; if it is greater, jump to end
        strbi $0, @string   ; otherwise write the value in register 0 to the address specified by label string
        jmpi @loop          ; return to start of loop
end:    hlt                 ; finishes executing
```
```
user@artixpc> ./rvm run test.asm                                                                                                                                                                                                                                                   22:39
a
b
[ .. ]
y
z
Halting!


final program:
45 50 49 45     00 00 00 00     
[ .. ]  
00 40 A0 00     42 00 00 00     
00 00   

final registers:
0000007B 00000000 00000000 00000000     00000000 00000000 00000000 00000000     
[ .. ]  
00000000 00000000 00000000 00000000     00000000 00000000 00000000 00000000     
Equality register: true
```
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
| instruction | short description         | opcode (hex) | example     | meaning         |
|-------------|---------------------------|--------------|-------------|-----------------|
| LDBI        | load byte immediate       | 01           | LDBI $1,0   | $1 <- 0         |
| LDBD        | load byte direct          | 01           | LDBD $1,0   | $1 <- MEM[0]    |
| LDBR        | load byte register        | 01           | LDBR $1,$0  | $1 <- MEM[$0]   |
| LDHI        | load half-word immediate  | 02           | LDHI $1,0   | $1 <- 0         |
| LDHD        | load half-word direct     | 02           | LDHD $1,0   | $1 <- MEM[0..2] |
| LDHR        | load half-word register   | 02           | LDHR $1,$0  | $1 <- MEM[$0]   |
| LDWD        | load word direct          | 03           | LDWD $1,0   | $1 <- MEM[0..4] |
| LDWR        | load word register        | 03           | LDWR $1,$0  | $1 <- MEM[$0]   |
| STRBI       | store byte immediate      | 04           | STRBI $1,0  | MEM[0] <- $1    |
| STRBR       | store byte register       | 04           | STRBR $1,$0 | MEM[$0] <- $1   |
| STRHI       | store half-word immediate | 05           | STRHI $1,0  | MEM[0..2] <- $1 |
| STRHR       | store half-word register  | 05           | STRHR $1,$0 | MEM[$0] <- $1   |
| STRWI       | store word immediate      | 06           | STRWI $1,0  | MEM[0..4] <- $1 |
| STRWR       | store word register       | 06           | STRWR $1,$0 | MEM[$0] <- $1   |
| MOV         | move register             | 07           | MOV $0,$1   | $0 <- $1        |

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

### Jumps
| instruction | short description           | opcode (hex) | example   | meaning                                  |
|-------------|-----------------------------|--------------|-----------|------------------------------------------|
| JMPI        | jump immediate              | 28           | JMPI 10   | pc <- 10                                 |
| JMPD        | jump direct                 | 28           | JMPD 10   | pc <- MEM[10..14]                        |
| JMPR        | jump register               | 28           | JMPR $0   | pc <- $0                                 |
| JMPEI       | jump if equal immediate     | 29           | JMPEI 10  | if equality_register: pc <- 10           |
| JMPED       | jump if equal direct        | 29           | JMPED 10  | if equality_register: pc <- MEM[10..14]  |
| JMPER       | jump if equal register      | 29           | JMPER $0  | if equality_register: pc <- $0           |
| JMPNEI      | jump if not equal immediate | 2A           | JMPNEI 10 | if !equality_register: pc <- 10          |
| JMPNED      | jump if not equal direct    | 2A           | JMPNED 10 | if !equality_register: pc <- MEM[10..14] |
| JMPNER      | jump if not equal register  | 2A           | JMPNER $0 | if !equality_register: pc <- $0          |

### Special
| instruction | short description     | opcode (hex) | example  | meaning                                 |
|-------------|-----------------------|--------------|----------|-----------------------------------------|
| PRTSD       | print string direct   | 30           | PRTSD 64 | prints string from MEM[64..] until null |
| PRTSR       | print string register | 30           | PRTSR $0 | prints string from MEM[$0..] until null |
