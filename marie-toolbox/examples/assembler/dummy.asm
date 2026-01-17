..section code 0x10             -- section with a code which should be executed
JUMP 100
HALT 200
CLEAR 2
ADD 50
a: add 500                      -- label address evaluated on the distance between section address
   subt 200
   jump a
..section code 0x50             -- section with a code which should be executed
HALT
HALT
..section data 0x60             -- section with a data - it shall not be executed
b:   ..hex 60                   -- ..hex is not assembly instruction but macro for assembly
c:   ..dec 80                   -- ..dec is not assembly instruction but macro for assembly
d:   ..bin 1101                 -- ..bin is not assembly instruction but macro for assembly
