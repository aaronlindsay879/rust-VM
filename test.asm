.data
string: .asciiz "hi"
.code
        prtsd @string
        ldhi $0, @string
        addi $0, 2
        addi $1, 0x69
        strbr $1, $0
        prtsd @string