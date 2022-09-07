-instructions in decoder

    // reg vx
    0x6XNN : set register VX
    0x7XNN : add nn to register VX no carry

    // index
    0xANNN : set index register to nnn

    // display
    0x00E0 : clear display
    0xDXYN : draw display

    // subroutines and jumps
    0x00EE : return from subroutine 
    0x2NNN : call subroutine
    0x1NNN : jump pc to nnn

    // skips
    0x3XNN : skip if reg VX == NN
    0x4XNN : skip if reg VX != NN
    0x5XY0 : skip if reg VX == reg VY
    0x9XY0 : skip if reg VX != reg VY


-instructions to put in decoder BUT HAVE FUNCS

    0x8XY1 : binary or
    0x8XY2 : binary and
    0x8XY3 : binary xor

    0x8XY4 : add with carry

    0x8XY5 : subtract with carry
    0x8XY7 : subtract with carry backwards 

    0x8XY6 : shift one bit to the right 
    0x8XYE : shift one bit to the left

    0xBNNN : jump with offset

    0xCXNN : random


-instructions to implement functions for and put in the decoder

    0xDXYN : draw display

    0xEX9E : skip if key
    0xEX9E : skip if key