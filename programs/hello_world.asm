start:

write_char:
    mov z, x
    out 0
    mov z, y
    out 1
    sep 0    ; in the simulation delay doesn't really matter, but in real life we would want something here
    rsp 0