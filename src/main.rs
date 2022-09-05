use chip_8_emulator::Chip8;
use rand::random;

fn main() {
    let mut x = Chip8::new();

    x.set_vx_reg_instruction(0, 128);

    println!("{}",x.general_regs[0]);
    
    x.shift_vx_register(0, false);
    println!("{}",x.general_regs[0]);

    x.shift_vx_register(0, true);
    println!("{}",x.general_regs[0]);









    // x.load_rom(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testrom.chip8"));
    // print!("");

    // x.start_processor_loop();
    // print!("");



}
