use chip_8_emulator::{Chip8, BinaryOp};

fn main() {
    let mut x = Chip8::new();

    // const TEST_REG : usize = 0;

    // x.set_vx_reg_instruction(TEST_REG, 15);
    // // x.set_vx_reg_instruction(1, 7);

    // x.bin_op_vx_reg_instruction(TEST_REG, 10, BinaryOp::Xor);

    // println!("{}", x.general_regs[TEST_REG]);
    // print!("");







    x.load_rom_from_radix(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testrom.chip8"));
    print!("");

    x.start_processor_loop();
    print!("");



}
