start:
    ssj
    ssf
    ldi x 0x4
    ldi y 0x8
    brn write_char
    ldi x 0x6
    ldi y 0x9
    brn write_char
    rsj
end:
    brn end
write_char:
    mov z x
    out 0
    mov z y
    out 1
    sep 0    ; in the simulation delay doesn't really matter, but in real life we would want something here
    rsp 0
    ret