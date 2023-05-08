.data
    hello: .asciiz 'Hello'
    world: .asciiz 'world!'
.code
loop:
    add 2,$0,$0
    djmp @loop