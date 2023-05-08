## Directives 

| directive name      | implemented | action                                                                                                   |
|---------------------|-------------|----------------------------------------------------------------------------------------------------------|
| .align <n>          | &#9744;     | aligns the next data directive on a 2^n boundary, if not specified then all default alignment is 4 bytes |
| .ascii <string>     | &#9745;     | stores a non-null terminated string                                                                      |
| .asciiz <string>    | &#9745;     | stores a null terminated string                                                                          |
| .byte <b1, ..., bn> | &#9744;     | stores n bytes in successive locations                                                                   |
| .half <h1, ..., hn> | &#9744;     | stores n half-words (2 bytes) in successive locations                                                    |
| .word <w1, ..., wn> | &#9744;     | stores n words (4 bytes) in successive locations                                                         |
| .space <n>          | &#9744;     | leaves n bytes free                                                                                      |
| .data               | &#9745;     | marks the start of the data section                                                                      |
| .code               | &#9745;     | marks the start of the code section                                                                      |
