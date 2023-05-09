.data
        .align 2
string: .asciiz "a"
.code
loop:   prtsd @string ; prints char
        ldbd $0, @string
        addi $0, 1
        gti $0, 0x7a ; load char, increment, check if greater than 0x7a ('z')
        jmpei @end ; if greater, jump to end
        strbi $0, @string
        jmpi @loop ; otherwise write character to memory and restart loop
end:    hlt

