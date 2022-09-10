use rand::{Rng, thread_rng, rngs::ThreadRng};
use std::{
    fs::File,
    io::BufRead,
    io::BufReader,
    thread,
    time,
};
use crate::{BinaryOp, Chip8Window};

pub struct Chip8 {
    // memory for the chip8 should be 4k
    // since I am using u16s for the instructions the size of the vector is 4096 / 2 = 2048
    // program memory should be full of nop statements by default wich are 0x0000
    pub memory : Vec<u8>,
    
    // the display is monochrome so the bytes representing pixels can just be bools
    // the display size should be 32 by 64
    pub display_buffer : Vec<Vec<bool>>,

    pub window : Chip8Window,

    // the specs given don't say how many stack entries there should be but I put 16
    pub stack : Vec<u16>,
    
    // 32 bit register for program counter
    pub pc_reg : u16,

    // 16 bit register for memory index register
    pub index_reg : u16,

    // 8 bit registers
    pub delay_timer_register : u8,
    pub sound_timer_register : u8,
    
    // flag registers
    pub vf_flag_reg : bool,
    pub jumped_flag_reg : bool,
    pub carry_flag_register : bool,

    // 16 general purpose 8 bit registers 
    pub general_regs : Vec<u8>,

    pub rng : ThreadRng,
}

impl Chip8 {
    const SCREEN_HEIGHT : usize = 32;
    const SCREEN_WIDTH : usize = 64;
    const PROGRAM_MEMORY_SIZE : usize = 4096;
    const CLOCK_SLEEP_TIME_SECONDS : f64 = 1.0 / 700.0;

    // 16bit bitmasks
    const FXXX_BITMASK : u16 = 0xF000;
    const XFXX_BITMASK : u16 = 0x0F00;
    const XXFX_BITMASK : u16 = 0x00F0;
    const XXXF_BITMASK : u16 = 0x000F;
    const XFFF_BITMASK : u16 = 0x0FFF;
    const XXFF_BITMASK : u16 = 0x00FF;

    // 8bit bitmasks
    const LEFTMOST_BITMASK : u8 = 0x80;
    const RIGHTMOST_BITMASK : u8 = 0x01;

    pub fn new() -> Chip8 {
        Chip8 {
            memory : vec![0; Self::PROGRAM_MEMORY_SIZE],
            display_buffer : vec![vec![false; Self::SCREEN_HEIGHT]; Self::SCREEN_WIDTH],
            window : Chip8Window::new(),
            stack : Vec::new(),
            pc_reg : 0,
            index_reg : 0,
            delay_timer_register : 0,
            sound_timer_register : 0,
            vf_flag_reg : false, 
            jumped_flag_reg : false,
            carry_flag_register : false,
            general_regs : vec![0; 16],
            rng : thread_rng()
        }
    }

    pub fn load_rom_from_radix(&mut self, file_path : &String) {
        let file_open_result = File::open(file_path);

        let file_handle = match file_open_result {
            Ok(file) => { file } 
            _ => { panic!("error : could not load file at specified path!") } 
        };

        let reader = BufReader::new(file_handle);
        let mut index = 0;
        for (line_index, line) in reader.lines().enumerate() {
            if line_index > Self::PROGRAM_MEMORY_SIZE - 1 {
                panic!("error : your program is too large! 4096 bytes of program memory maximum")
            }

            let line = line.unwrap();

            // figure out why this has to be 16 in the second arg
            let instruction_result = u16::from_str_radix(&line, 16);
            
            let instruction = match instruction_result {
                Ok(hex_number) => { hex_number }
                _ => { panic!("error : instructions must be in radix hex like 'XXXX' !") }
            };

            self.memory[index as usize] = (instruction >> 8) as u8; 
            self.memory[index as usize + 1] = (instruction & Self::XXFF_BITMASK) as u8; 
            index += 2;
        }
    }

    /// this is the main processor loop for the chip 8 
    /// 
    /// the loop works as follows :
    /// fetch -> decode -> execute -> incf pc 
    /// 
    /// the loop will run at a fixed speed of 1mhz simulated
    pub fn start_processor_loop(&mut self) {
        loop {
            // handle the input
            // TODO this should return something
            self.window.handle_input();

            let instruction_start_time = time::Instant::now();

            // if the program counter has run out of instructions break out of the processor loop
            if self.pc_reg as usize > Self::PROGRAM_MEMORY_SIZE - 1 { break }

            // decode and execute will both read the contents of the instruction and then execute the instruction afterwards
            let instruction = self.fetch();

            self.decode_and_execute(instruction);

            // if there was a jump (or later on probably a call) dont increment the program counter
            if !self.jumped_flag_reg {
                self.pc_reg += 2;
            } else {
                self.jumped_flag_reg = false;
            }
            
            let operation_duration = Self::CLOCK_SLEEP_TIME_SECONDS -instruction_start_time.elapsed().as_secs_f64();
            if operation_duration > 0.0 {
                thread::sleep(time::Duration::from_secs_f64(operation_duration));
            }

            //draw the window
            self.window.draw_canvas(self.display_buffer.clone());
        }
    }

    pub fn fetch(&self) -> u16 {
        let opcode = self.memory[self.pc_reg as usize];
        let operand = self.memory[self.pc_reg as usize + 1];
        ((opcode as u16) << 8) + operand as u16
    }

    /// this is a function that will debug and execute a single instruction
    /// 
    /// if the instruction does not match anything in the specified instruction list then it will act as a NOP
    pub fn decode_and_execute(&mut self, instruction : u16) {
        match instruction {
            // 0x00E0 (clear screen) 
            0x00E0 => self.clear_display_instruction(),

            // 0x1NNN (jumps the program counter to a specific location)
            i if i & Self::FXXX_BITMASK == 0x1000 => {
                // call the jump instruction with NNN from 1NNN to tell the program counter where to jump
                let location = i & Self::XFFF_BITMASK;
                self.jump_instruction(location)
            }

            // 0x6XNN(set register VX)
            i if i & Self::FXXX_BITMASK == 0x6000 => {
                //
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;
                let num = (i & Self::XXFF_BITMASK) as u8;
                self.set_vx_reg_instruction(reg, num);
            }

            // ---- TODO MAKE SURE THIS COVERS ADD WITH CARRY ----
            //
            // add value to register vx0
            i if i & Self::FXXX_BITMASK == 0x7000 => {
                //
                let reg = (i & Self::XFXX_BITMASK) >> 8;
                let num = i & Self::XXFF_BITMASK;
                self.add_reg_vx_instruction(reg as usize, num as u8, false);
            }

            // set index register i
            i if i & Self::FXXX_BITMASK == 0xA000 => {
                let number = i & Self::XFFF_BITMASK;
                self.set_index_reg_instruction(number)
            }
            
            // draw/display 
            //TODO IMPLEMENT DRAW DISPLAY
            i if i & Self::FXXX_BITMASK == 0xD000 => self.draw_display_instruction(),

            // TODO return
            0x00EE => self.return_instruction(),

            // TODO call
            i if i & Self::FXXX_BITMASK == 0x2000 => { 
                let location = i & Self::XFFF_BITMASK;
                self.call_instruction(location);
            }

            // TODO skip if vx is equal to nn
            // for instructions : 3XNN 4XNN 5XY0 9XY0
            i if i & Self::FXXX_BITMASK == 0x3000 || 
                 i & Self::FXXX_BITMASK == 0x4000 || 
                 i & Self::FXXX_BITMASK == 0x5000 || 
                 i & Self::FXXX_BITMASK == 0x9000 => {
                
                // the first reg value in the instruction will be the value of the register with the index of the 2nd nybble in the instruction
                let first_reg_value = self.general_regs[((i & Self::XFXX_BITMASK) >> 8) as usize];
                
                // the second reg value in the instruction will be the value of the register with the index of the 3rd nybble in the instruction
                let second_reg_value = self.general_regs[((i & Self::XXFX_BITMASK) >> 4) as usize] as u8;

                // the number to compare the first register too will be the last 2 nybbles in the instruction
                let num = (i & Self::XXFF_BITMASK) as u8;

                // these instructions are almost all the same and have one function for them
                // the first item in each tuple returned here is the skip amount
                // the second item in each tuple is weather the skip should occur if the reg and number are the same or not
                match i & Self::FXXX_BITMASK {
                    0x3000 => self.skipif_vx_reg_nn_instruction(first_reg_value, num, true),
                    0x4000 => self.skipif_vx_reg_nn_instruction(first_reg_value, num, false),
                    0x5000 => self.skipif_vx_reg_nn_instruction(first_reg_value, second_reg_value, true),
                    0x9000 => self.skipif_vx_reg_nn_instruction(first_reg_value, second_reg_value, false),
                    _ => {}
                };
            }

            i if i & Self::FXXX_BITMASK == 0x8000 => {
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;
                let num = self.general_regs[((i & Self::XXFX_BITMASK) >> 4) as usize];

                match i & Self::XXXF_BITMASK {
                    // binary operation instructions
                    0x0001 => self.bin_op_vx_reg_instruction(reg, num, BinaryOp::Or),
                    0x0002 => self.bin_op_vx_reg_instruction(reg, num, BinaryOp::And),
                    0x0003 => self.bin_op_vx_reg_instruction(reg, num, BinaryOp::Xor),
                    // add with carry
                    0x0004 => self.add_reg_vx_instruction(reg, num, true),
                    // subtract with carry
                    0x0005 => self.subtract_vx_reg_instruction(reg, num, false),
                    // right shift instruction
                    0x0006 => self.shift_vx_register(reg, true),
                    // subtract with carry backwards
                    0x0007 => self.subtract_vx_reg_instruction(reg, num, true),
                    // left shift instruction
                    0x000E => self.shift_vx_register(reg, false),
                    _ => {}
                }
            }

            i if i & Self::FXXX_BITMASK == 0xB000 => {
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;
                let offset = i & Self::XXFF_BITMASK;
                self.jump_with_offset_instruction(reg, offset)
            }

            i if i & Self::FXXX_BITMASK == 0xC000 => {
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;
                let num = (i & Self::XXFF_BITMASK) as u8;
                self.random_instruction(reg, num);
            }

            _ => {}
        }
    }

    /// this will set every byte storing info for the display to off
    /// 
    /// in decoder
    /// 
    /// for instructions : 00E0
    pub fn clear_display_instruction(&mut self) {
        self.display_buffer = vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT];
    }

    /// this will just set the program counter to a specific location in program memory of NNN
    /// 
    /// in decoder
    /// 
    /// for instructions : 1NNN
    pub fn jump_instruction(&mut self, location : u16) {
        self.pc_reg = location;
        self.jumped_flag_reg = true
    }

    /// this sets the index register to a specific number
    /// 
    /// in decoder
    /// 
    /// for instructions : ANNN
    pub fn set_index_reg_instruction(&mut self, num : u16) {
        self.index_reg = num;
    }

    /// 
    /// 
    /// in decoder
    /// 
    /// for instructions : DXYN
    pub fn draw_display_instruction(&self) {
        // TODO : implement this later but i would update it to draw to an actual window



    }














    
    ///
    /// 
    /// in decoder
    /// 
    /// for instructions : EE00
    pub fn return_instruction(&mut self) {
        let return_address = self.stack.pop();
        if let None = return_address {
            panic!("error : stack underflow");
        }
        self.pc_reg = return_address.unwrap();
        self.jumped_flag_reg = true;
    }

    /// 
    /// 
    /// in decoder
    /// 
    /// for instructions : 2NNN 
    pub fn call_instruction(&mut self, location : u16) {
        // + 2 to make sure that it executes the NEXT instruction once a return is hit
        self.stack.push(self.pc_reg + 2);
        self.pc_reg = location;
        self.jumped_flag_reg = true
    }

    /// 
    /// 
    /// in decoder
    /// 
    /// for instructions : 3XNN 4XNN 5XY0 9XY0
    pub fn skipif_vx_reg_nn_instruction(&mut self, reg_val : u8, num : u8, equality : bool) {
        // this is a tricky way to have one function handle 4 instructions
        // if you have the register equal to the number and you do want them to be equal equality will be true and this is true
        // if they are not equal and you do not want them to be equal this will go through
        if !((reg_val == num) ^ equality) {
            self.pc_reg += 2
        }
    }

    ///
    /// 
    /// in decoder
    /// 
    /// for instructions : 6XNN
    pub fn set_vx_reg_instruction(&mut self, reg : usize, num : u8) {
        self.general_regs[reg] = num
    }

    /// 
    /// 
    /// NOT in decoder
    /// 
    /// for instructions : 8XY1 8XY2 8XY3
    pub fn bin_op_vx_reg_instruction(&mut self, reg : usize, num : u8, op : BinaryOp) {
        self.general_regs[reg] = match op {
            BinaryOp::Or => self.general_regs[reg] | num,
            BinaryOp::And => self.general_regs[reg] & num,
            // this MIGHT be wrong check later (probably fine tho because I think there is only binary xor no logical)
            BinaryOp::Xor => self.general_regs[reg] ^ num,
        };
    }

    ///
    /// 
    /// in decoder but NOT FOR 8xy4
    /// 
    /// for instructions : 8XY4 7XNN 
    pub fn add_reg_vx_instruction(&mut self, reg : usize, num : u8, carry : bool) {
        // wrapping add will add and account for overflows
        if carry {
            match self.general_regs[reg].checked_add(num) {
                Some(_number) => self.vf_flag_reg = false,
                _ => self.vf_flag_reg = true
            }
        }

        self.general_regs[reg] = self.general_regs[reg].wrapping_add(num)
    }

    ///
    /// 
    /// for instructions : 8XY5 8XY7
    pub fn subtract_vx_reg_instruction(&mut self, reg : usize, num : u8, flipped : bool) {
        self.vf_flag_reg = true;
        self.general_regs[reg] = if flipped {
            match num.checked_sub(self.general_regs[reg]) {
                Some(result) => result,
                None => {
                    self.vf_flag_reg = false;
                    num.wrapping_sub(self.general_regs[reg])
                }
            }
        } else {
            match self.general_regs[reg].checked_sub(num) {
                Some(result) => result, 
                None => {
                    self.vf_flag_reg = false;
                    self.general_regs[reg].wrapping_sub(num)
                }
            }
        }
    }

    ///
    /// 
    /// for instructions : 8XY6 8XYE
    pub fn shift_vx_register(&mut self, reg : usize, right_shift : bool) {
        self.general_regs[reg] = if right_shift {
            self.vf_flag_reg = (self.general_regs[reg] & Self::RIGHTMOST_BITMASK) != 0; 
            self.general_regs[reg] >> 1
        } else {
            self.vf_flag_reg = (self.general_regs[reg] & Self::LEFTMOST_BITMASK) != 0; 
            self.general_regs[reg] << 1
        }
    }

    ///
    /// 
    /// for instructions : BXNN
    pub fn jump_with_offset_instruction(&mut self, reg : usize, offset : u16) {
        self.pc_reg += offset;
        self.pc_reg += self.general_regs[reg] as u16; 
        self.jumped_flag_reg = true;
    }

    ///
    /// 
    /// for instructions : CXNN
    pub fn random_instruction(&mut self, reg : usize, num : u8) {
        self.general_regs[reg] = self.rng.gen_range(0..u8::MAX) & num;
    }
}