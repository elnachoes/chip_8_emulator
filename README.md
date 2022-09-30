# rip8

This is a chip 8 emulator I wrote for fun in Rust and OpenGL with sdl2. 

The emulator works for the most part at this point in time and is compatible with most Chip8 games however there is no audio beep yet and the input is a little weird and I am working on a fix for both

Setup and usage is very simple. You will need a rust compiler and sdl2 installed. After that running the program requires a clockspeed in hz and a rom path

EX : rip8 clockspeed(hz) RomPath

here is the emulator running a game called "pumpkindressup.ch8" by "SystemLogoff" : (https://johnearnest.github.io/chip8Archive/play.html?p=pumpkindressup)
![image](https://user-images.githubusercontent.com/31595608/193350022-a1f39966-21f6-4a49-98a0-153e8bf704f5.png)
