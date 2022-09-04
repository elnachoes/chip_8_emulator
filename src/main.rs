use chip_8_emulator::Chip8;

fn main() {
    let mut x = Chip8::new();
    // print!("");
    // x.decode_and_execute(0x66aa);
    // print!("");
    // x.decode_and_execute(0x7601);
    // print!("");
    // x.decode_and_execute(0x1111);
    // print!("");
    
    x.load_rom(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testrom.chip8"));
    print!("");

    x.start_processor_loop();
    print!("");
    // let x = i16::from_str_radix("0f", 16).unwrap();

    // print!("");
    // print!("{}", 0x1010 & 0xF000);
}
