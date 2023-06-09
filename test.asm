.data
        .align 2            ; align following directive to 2 bytes
string: .asciiz "a"         ; store the null-terminated string "a"
.code
loop:   prtsd @string       ; prints string stored at label string
        ldbd $0, @string    ; loads the byte located at the label string into register 0
        addi $0, 1          ; adds 1 to register 0
        gti $0, 'z'         ; check if the new value in register 0 is greater than 'z' (the half-word 0x007A)
        jmpei @end          ; if it is greater, jump to end
        strbi $0, @string   ; otherwise write the value in register 0 to the address specified by label string
        jmpi @loop          ; return to start of loop
end:    hlt                 ; finishes executing

