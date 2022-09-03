use chip_8_emulator::Chip8;

fn main() {
    let mut x = Chip8::new();
    x.decode(0x1010);

    // print!("{}", 0x1010 & 0xF000);
}
