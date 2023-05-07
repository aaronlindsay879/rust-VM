.data
    hello: .asciiz 'Hello'
    world: .asciiz 'vorld!'
.code
    load $0 @world
    inc $0
    store $0 @world

    PRTS @hello
    PRTS @world