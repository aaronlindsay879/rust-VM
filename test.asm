.data
secret: .word 0x706f6700
.code
        load $1, 10
loop:   prts @secret
        inc $0
        eq $0, $1
        djmpne @loop
        hlt