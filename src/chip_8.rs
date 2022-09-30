use rand::{
    Rng, 
    thread_rng, 
    rngs::ThreadRng
};

use std::{
    fs::File,
    io::{BufRead, Read},
    io::BufReader,
    vec,
};

use crate::{
    BinaryOp, 
    Keyboard,
    Font
};

pub struct Chip8 {
    // memory for the chip8 should be 4k
    // since I am using u16s for the instructions the size of the vector is 4096 / 2 = 2048
    // program memory should be full of nop statements by default wich are 0x0000
    pub memory : Vec<u8>,
    
    // the display is monochrome so the bytes representing pixels can just be bools
    // the display size should be 32 by 64
    pub display_buffer : Vec<Vec<bool>>,

    // the specs given don't say how many stack entries there should be but I put 16
    pub stack : Vec<u16>,
    
    // 32 bit register for program counter
    pub pc_reg : u16,

    // 16 bit register for memory index register
    pub index_reg : u16,

    // 8 bit registers
    pub delay_timer_register : u8,
    pub sound_timer_register : u8,
    
    // 16 general purpose 8 bit registers 
    pub v_regs : Vec<u8>,

    // current keyboard state
    pub keyboard : Keyboard,

    // rng for the rng function
    pub rng : ThreadRng,

    // font data
    pub font : Font,
}

impl Chip8 {
    const SCREEN_HEIGHT : usize = 32;
    const SCREEN_WIDTH : usize = 64;
    const PROGRAM_MEMORY_SIZE : usize = 4096;

    // 16bit bitmasks
    const FXXX_BITMASK : u16 = 0xF000;
    const XFXX_BITMASK : u16 = 0x0F00;
    const XXFX_BITMASK : u16 = 0x00F0;
    const XXXF_BITMASK : u16 = 0x000F;
    const XFFF_BITMASK : u16 = 0x0FFF;
    const XXFF_BITMASK : u16 = 0x00FF;

    // 8bit bitmasks
    const BIT1_BITMASK : u8 = 0b0000_0001;
    const BIT8_BITMASK : u8 = 0b1000_0000;

    pub fn new() -> Chip8 {
        Chip8 {
            memory : vec![0; Self::PROGRAM_MEMORY_SIZE],
            display_buffer : vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT],
            stack : Vec::new(),
            pc_reg : 512,
            index_reg : 0,
            delay_timer_register : 0,
            sound_timer_register : 0,
            v_regs : vec![0; 16],
            keyboard : Keyboard::None,
            rng : thread_rng(),
            font : Font::new_standard()
        }
    }


    /// this fn will load a rom from a binary file into the chip 8's memory
    /// 
    /// TODO : handle rom being to large
    pub fn load_rom_from_bin(&mut self, file_path : &String) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let mut index = self.pc_reg as usize;
        for i in file.bytes() {
            match i {
                Ok(byte) => self.memory[index] = byte,
                _ => break
            }
            index += 1
        }
    }

    ///  this fn will load a rom at location 512 in memory to allow for font and sprite space at the beggining of memory
    ///
    /// this fn is mostley for debugging and writing programs in hex to test things
    /// 
    /// this loads a program that looks like this with comments starting with ';;' :
    /// 
    /// ;; registers to set the font location 
    /// 600c
    /// 6105
    /// 
    /// ;; put the font location here
    /// f029
    /// 
    /// ;; draw the font char
    /// d015
    pub fn load_rom_from_radix(&mut self, file_path : &String) -> Result<(), String>{
        let file_open_result = File::open(file_path);

        let file_handle = match file_open_result {
            Ok(file) => { file } 
            _ => { return Err(String::from("error : could not load file at specified path!")) } 
        };
        
        // the rom will be loaded and started at location 512
        let mut index = self.pc_reg as usize;
        
        let reader = BufReader::new(file_handle);
        for (_line_index, line) in reader.lines().enumerate() {
            if index > Self::PROGRAM_MEMORY_SIZE - 1 {
                return Err(String::from("error : your program is too large! 4096 bytes of program memory maximum"))
            }

            let line = line.unwrap();

            if line.starts_with(";;") || line.is_empty() {
                continue
            }

            // figure out why this has to be 16 in the second arg
            let instruction_result = u16::from_str_radix(&line, 16);
            
            let instruction = match instruction_result {
                Ok(hex_number) => { hex_number }
                _ => { return Err(String::from("error : instructions must be in radix hex like 'XXXX' !")) }
            };

            self.memory[index as usize] = (instruction >> 8) as u8; 
            self.memory[index as usize + 1] = (instruction & Self::XXFF_BITMASK) as u8; 
            index += 2;
        }

        Ok(())
    }

    /// this fn loads a font into memory based on the font object given in the ctor
    pub fn load_font(&mut self) {
        let mut font_data_index : usize = self.font.font_location_in_memory;

        for i in self.font.font_data.iter() {
            self.memory[font_data_index] = *i;
            font_data_index += 1;
        }
    }

    /// this fn is for calling one frame of the procesor 
    /// 
    /// will return true if there is still memory left to read
    /// 
    /// will return false if there is no memory left to read
    pub fn processor_frame(&mut self, keyboard : Keyboard) -> bool {
        if self.pc_reg as usize > Self::PROGRAM_MEMORY_SIZE - 1 { 
            return false;
        }
        
        let instruction = self.fetch();

        self.keyboard = keyboard;

        self.decode_and_execute(instruction);

        true
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

            // set index register i instruction
            i if i & Self::FXXX_BITMASK == 0xA000 => {
                let number = i & Self::XFFF_BITMASK;
                self.set_index_reg_instruction(number)
            }
            
            // draw sprite instruction
            i if i & Self::FXXX_BITMASK == 0xD000 => {
                let x_coordinate = self.v_regs[((i & Self::XFXX_BITMASK) >> 8) as usize];
                let y_coordinate = self.v_regs[((i & Self::XXFX_BITMASK) >> 4) as usize];
                let sprite_height = (i & Self::XXXF_BITMASK) as u8;
                self.draw_sprite_instruction(x_coordinate, y_coordinate, sprite_height);
            },

            // return instruction
            0x00EE => self.return_instruction(),

            // call instruction
            i if i & Self::FXXX_BITMASK == 0x2000 => { 
                let location = i & Self::XFFF_BITMASK;
                self.call_instruction(location);
            }

            // for instructions : 3XNN 4XNN 5XY0 9XY0
            i if i & Self::FXXX_BITMASK == 0x3000 || 
                 i & Self::FXXX_BITMASK == 0x4000 || 
                 i & Self::FXXX_BITMASK == 0x5000 || 
                 i & Self::FXXX_BITMASK == 0x9000 || 
                 i & Self::FXXX_BITMASK == 0xE000 => {
                
                // the first reg value in the instruction will be the value of the register with the index of the 2nd nybble in the instruction
                let first_reg_value = self.v_regs[((i & Self::XFXX_BITMASK) >> 8) as usize];
                
                // the second reg value in the instruction will be the value of the register with the index of the 3rd nybble in the instruction
                let second_reg_value = self.v_regs[((i & Self::XXFX_BITMASK) >> 4) as usize] as u8;

                // keyboard keycode
                let key_code = self.keyboard.get_keycode();

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

                    // TODO FIX THIS SOMETHING IS WRONG HERE
                    j if j == 0xE000 => match i & Self::XXFF_BITMASK {
                        0x009E => self.skipif_vx_reg_nn_instruction(first_reg_value, key_code, true),
                        0x00A1 => self.skipif_vx_reg_nn_instruction(first_reg_value, key_code, false),
                        _ => {}
                    },

                    _ => {}
                };
            }

            i if i & Self::FXXX_BITMASK == 0x8000 => {
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;
                let num = self.v_regs[((i & Self::XXFX_BITMASK) >> 4) as usize];

                match i & Self::XXXF_BITMASK {
                    0x0000 => self.set_vx_reg_instruction(reg, num),

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

            i if i & Self::FXXX_BITMASK == 0xF000 => {
                let reg = ((i & Self::XFXX_BITMASK) >> 8) as usize;

                match i & Self::XXFF_BITMASK {
                    0x0007 => self.set_vx_reg_instruction(reg, self.delay_timer_register),
                    0x0015 => self.set_delay_timer_reg_instruction(self.v_regs[reg]),
                    0x0018 => self.set_sound_timer_reg_instruction(self.v_regs[reg]),
                    0x001E => self.add_to_index_reg_instruction(self.v_regs[reg] as u16),
                    0x000A => self.get_key_instruction(reg),
                    0x0029 => self.set_index_to_font_char_instruction(self.v_regs[reg] as usize),
                    0x0033 => self.bcd_instruction(reg),
                    0x0055 => self.store_to_memory_instruction(reg),
                    0x0065 => self.load_from_memory_instruction(reg),
                    _ => {}
                }
            }

            _ => {}
        }
    }

    /// this fn is for updating the timers so that the timers can decrement once per frame which is detached from the chip8 clock 
    pub fn update_timers(&mut self) {
        if self.delay_timer_register != 0 {
            self.delay_timer_register -= 1
        }

        if self.sound_timer_register != 0 {
            self.sound_timer_register -= 1
        }
    }



    /// this fn will set every byte storing info for the display to off
    /// 
    /// for instructions : 00E0
    pub fn clear_display_instruction(&mut self) {
        self.display_buffer = vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT];
        self.pc_reg += 2
    }

    /// this fn will just set the program counter to a specific location in program memory of NNN
    /// 
    /// for instructions : 1NNN
    pub fn jump_instruction(&mut self, location : u16) {
        self.pc_reg = location;
        // self.jumped_flag_reg = true
    }

    /// this fn sets the index register to a specific number
    /// 
    /// for instructions : ANNN
    pub fn set_index_reg_instruction(&mut self, num : u16) {
        self.index_reg = num;
        self.pc_reg += 2
    }

    /// this fn will add a number to the index register
    /// 
    ///  for instructions 
    pub fn add_to_index_reg_instruction(&mut self, num : u16) {
        self.index_reg = self.index_reg.wrapping_add(num);
        self.pc_reg += 2
    }

    /// this fn will set the value in the delay timer reg
    ///
    /// for instructions 
    pub fn set_delay_timer_reg_instruction(&mut self, num : u8) {
        self.delay_timer_register = num;
        self.pc_reg += 2
    }

    /// this fn will set the value in the sound timer reg
    /// 
    /// for instructions 
    pub fn set_sound_timer_reg_instruction(&mut self, num : u8) {
        self.sound_timer_register = num;
        self.pc_reg += 2
    }

    /// this fn will keep blocking by not incrementing the pc while it waits for input
    /// 
    /// for instructions 
    pub fn get_key_instruction(&mut self, reg : usize) {
        // let keyboard_result = self.window.handle_input();
        match self.keyboard.get_keycode() {
            key_code if key_code <= 0xf => {
                self.v_regs[reg] = key_code;
                self.pc_reg += 2
            },

            // if there was no key pressed dont advance the pc so that this instruction keeps executing
            _ => {}
        }
    }

    /// this fn will draw a sprite at a given x and y coordinate to a given sprite height
    /// 
    /// the vf register will get flipped 
    /// 
    /// for instructions : DXYN
    pub fn draw_sprite_instruction(&mut self, x_coordinate : u8, y_coordinate : u8, sprite_height : u8) {

        // make sure that x coordinate is not running off the side of the screen
        let x_coordinate = x_coordinate % Self::SCREEN_WIDTH as u8;
        let y_coordinate = y_coordinate % Self::SCREEN_HEIGHT as u8;

        // this is the outer loop for the rows of the sprite
        'rows : for y_offset in 0..sprite_height {
            let sprite_row = self.memory[(self.index_reg + y_offset as u16) as usize];

            // because the rows are drawn on a display buffer that has the largest number to the right of the screen,
            // and the bytes are stored with the largest bits to the left (big endian),
            // this iterator needs to move in reverse for moving through the sprite row and move foward for translating to the display
            // TODO : look into double ended iterators to make this a bit cleaner
            let mut sprite_slice_iter = 0..8;
            
            // this is the loop for the individual columns  
            'columns : for shift_amount in sprite_slice_iter.clone().rev() {

                // this gets the individual bit for a given column in the sprite row
                let bit = (sprite_row >> shift_amount) & Self::BIT1_BITMASK;
                
                // the x offset should be the next item in the sprite slice iter 
                let x_offset = sprite_slice_iter.next().unwrap();

                // add the offsets to the x and y coordinates
                let x_coordinate = x_coordinate + x_offset;
                let y_coordinate = y_coordinate + y_offset;
                
                // if the x coordinate is greater than or equal to the screen width stop drawing the current column
                if x_coordinate >= Self::SCREEN_WIDTH as u8 {
                    break 'columns;
                
                // if the y coordinate is greater than or equal to the screen height stop drawing the whole sprite
                } else if y_coordinate >= Self::SCREEN_HEIGHT as u8 {
                    break 'rows;
                
                // if the bit is true, check if there was a pixel already shaded on that position in the display buffer
                // if there was a pixel already shaded on that position then set the shade to false and the vf flag reg to true
                // else shade that pixel
                } else if bit != 0 {
                    self.v_regs[0xf] = 0;
                    if self.display_buffer[(y_coordinate) as usize][(x_coordinate) as usize] {
                        self.v_regs[0xf] = 1;
                        self.display_buffer[(y_coordinate) as usize][(x_coordinate) as usize] = false;
                    } else {
                        self.display_buffer[(y_coordinate) as usize][(x_coordinate) as usize] = true;
                    }
                }
            }
        }
        self.pc_reg += 2
    }
    
    ///
    /// 
    /// for instructions : EE00
    pub fn return_instruction(&mut self) {
        let return_address = self.stack.pop();
        if let None = return_address {
            panic!("error : stack underflow");
        }
        self.pc_reg = return_address.unwrap();
    }

    /// 
    /// 
    /// for instructions : 2NNN 
    pub fn call_instruction(&mut self, location : u16) {
        // + 2 to make sure that it executes the NEXT instruction once a return is hit
        self.stack.push(self.pc_reg + 2);
        self.pc_reg = location;
    }

    /// 
    /// 
    /// for instructions : 3XNN 4XNN 5XY0 9XY0 EX9E EXA1
    pub fn skipif_vx_reg_nn_instruction(&mut self, reg_val : u8, num : u8, equality : bool) {
        // this is a tricky way to have one function handle 4 instructions
        // if you have the register equal to the number and you do want them to be equal equality will be true and this is true
        // if they are not equal and you do not want them to be equal this will go through
        if (reg_val == num) == equality {
            self.pc_reg += 4
        }
        else {
            self.pc_reg += 2
        }
    }

    ///
    /// 
    /// for instructions : 6XNN
    pub fn set_vx_reg_instruction(&mut self, reg : usize, num : u8) {
        self.v_regs[reg] = num;
        self.pc_reg += 2
    }

    /// 
    /// 
    /// for instructions : 8XY1 8XY2 8XY3
    pub fn bin_op_vx_reg_instruction(&mut self, reg : usize, num : u8, op : BinaryOp) {
        self.v_regs[reg] = match op {
            BinaryOp::Or => self.v_regs[reg] | num,
            BinaryOp::And => self.v_regs[reg] & num,
            // this MIGHT be wrong check later (probably fine tho because I think there is only binary xor no logical)
            BinaryOp::Xor => self.v_regs[reg] ^ num,
        };
        self.pc_reg += 2
    }

    /// 
    /// 
    /// for instructions : 8XY4 7XNN 
    pub fn add_reg_vx_instruction(&mut self, reg : usize, num : u8, carry : bool) {
        // wrapping add will add and account for overflows
        if carry {
            match self.v_regs[reg].checked_add(num) {
                Some(_number) => self.v_regs[0xf] = 0,
                _ => self.v_regs[0xf] = 1
            }
        }

        self.v_regs[reg] = self.v_regs[reg].wrapping_add(num);

        self.pc_reg += 2
    }

    /// this will do a subtraction on reg with a given num. 
    /// 
    /// for instructions : 8XY5 8XY7
    pub fn subtract_vx_reg_instruction(&mut self, reg : usize, num : u8, flipped : bool) {
        self.v_regs[0xf] = 1; 
        self.v_regs[reg] = if flipped {
            match num.checked_sub(self.v_regs[reg]) {
                Some(result) => result,
                None => {
                    self.v_regs[0xf] = 0; 
                    num.wrapping_sub(self.v_regs[reg])
                }
            }
        } else {
            match self.v_regs[reg].checked_sub(num) {
                Some(result) => result, 
                None => {
                    self.v_regs[0xf] = 0; 
                    self.v_regs[reg].wrapping_sub(num)
                }
            }
        };

        self.pc_reg += 2
    }

    /// this fn binary shifts the value in reg right and left
    /// 
    /// for instructions : 8XY6 8XYE
    pub fn shift_vx_register(&mut self, reg : usize, right_shift : bool) {
        self.v_regs[reg] = if right_shift {
            self.v_regs[0xf] = self.v_regs[reg] & Self::BIT1_BITMASK;
            self.v_regs[reg] >> 1
        } else {
            self.v_regs[0xf] = self.v_regs[reg] & Self::BIT8_BITMASK;
            self.v_regs[reg] << 1
        };

        self.pc_reg += 2
    }

    /// this fn will jump to a given location with the offset of whatever is in the given register usually v0
    /// 
    /// for instructions : BXNN
    pub fn jump_with_offset_instruction(&mut self, reg : usize, offset : u16) {
        self.pc_reg += offset;
        self.pc_reg += self.v_regs[reg] as u16; 
    }

    /// this fn sets reg vx to a random number
    /// 
    /// for instructions : cxnn
    pub fn random_instruction(&mut self, reg : usize, num : u8) {
        self.v_regs[reg] = self.rng.gen_range(0..u8::MAX) & num;
        self.pc_reg += 2
    }

    /// this will set the index register to the location of a font char's sprite
    /// 
    /// for instructions fx33
    pub fn set_index_to_font_char_instruction(&mut self, char : usize) {
        self.index_reg = self.font.char_sprite_locations[char];
        self.pc_reg += 2
    }

    /// this fn will store a bcd representation of reg x at the location in the index reg
    /// 
    /// for instructions fx33
    pub fn bcd_instruction(&mut self, reg : usize) {
        let reg_val = self.v_regs[reg];
        let index = self.index_reg as usize;

        let first_byte_hundreds = reg_val / 100;
        let second_byte_tens = (reg_val - (first_byte_hundreds * 100) ) / 10;
        let third_byte_ones = (reg_val - (first_byte_hundreds * 100) - (second_byte_tens * 10) ) / 1;

        self.memory[index] = first_byte_hundreds;
        self.memory[index + 1] = second_byte_tens;
        self.memory[index + 2] = third_byte_ones;
        
        self.pc_reg += 2
    }

    /// this fn dumps the contents of all of the v registers to a given location in memory up to a given reg
    /// 
    /// if you were to select reg vf it should store the contents of every single reg
    /// 
    /// for instructions fx55
    pub fn store_to_memory_instruction(&mut self, reg : usize) {
        for (i, val) in self.v_regs.iter().enumerate() {
            self.memory[self.index_reg as usize + i] = *val;
            if i == reg { break }
        }

        self.pc_reg += 2
    }

    /// this fn loads memory into the regs from the location of the value that the index reg is holding
    /// 
    /// this fn will only load values up to a given reg number so if you were to pick reg vf it will fill all 16 regs 
    /// 
    /// for instructions fx65
    pub fn load_from_memory_instruction(&mut self, reg : usize) {
        if self.v_regs[0] == 0 {
            self.v_regs[reg] = self.memory[self.index_reg as usize]
        } else {
            for i in self.index_reg as usize..=self.index_reg as usize + reg  {
                self.v_regs[i - self.index_reg as usize] = self.memory[i]
            }
        }

        self.pc_reg += 2
    }
}