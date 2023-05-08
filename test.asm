.data
    hello: .asciiz 'Hello'
    world: .asciiz 'world!'
.code
    load $0, 5
    load $1, 2
    loop: add $1, $3, $3
    inc $2
    eq $0, $2
    djmpne @loop
    hlt