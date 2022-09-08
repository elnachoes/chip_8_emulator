use chip_8_emulator::{Chip8, BinaryOp, Chip8Window};

fn main() {
    let mut x = Chip8::new();

    // const TEST_REG : usize = 0;

    // x.set_vx_reg_instruction(TEST_REG, 15);
    // // x.set_vx_reg_instruction(1, 7);

    // x.bin_op_vx_reg_instruction(TEST_REG, 10, BinaryOp::Xor);

    // println!("{}", x.general_regs[TEST_REG]);
    // print!("");







    // x.load_rom_from_radix(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testrom.chip8"));
    // print!("");

    // x.start_processor_loop();
    // print!("");





    // this will draw a little smily face for testing
    let mut x = Chip8Window::new();

    let mut newBuffer = vec![vec![false; 64]; 32];

    newBuffer[6][2] = true;
    newBuffer[8][2] = true;

    newBuffer[5][5] = true;
    newBuffer[6][6] = true;
    newBuffer[7][6] = true;
    newBuffer[8][6] = true;
    newBuffer[9][5] = true;

    x.buffer = newBuffer;
 
    x.run_window();


}
