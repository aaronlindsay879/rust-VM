.data
.code
loop:   addi $0,2
        addi $1,1
        eqi $1,5
        jmpnei @loop
        hlt