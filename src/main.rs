use chip_8_emulator::Chip8;

fn main() {
    let mut x = Chip8::new();
    print!("");
    x.decode(0x66aa);
    print!("");
    x.decode(0x7601);
    print!("");
    x.decode(0x1111);
    print!("");
    

    // print!("{}", 0x1010 & 0xF000);
}
