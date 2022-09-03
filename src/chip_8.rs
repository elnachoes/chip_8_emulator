
// blog used : https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

use core::num;

pub struct Chip8 {
    // memory for the chip8 should be 4k
    pub memory : Vec<u8>,
    
    // the display is monochrome so the bytes representing pixels can just be bools
    // the display size should be 32 by 64
    pub display : Vec<Vec<bool>>,

    // the specs given don't say how many stack entries there should be but I put 16
    pub stack : Vec<u16>,
    
    // 32 bit register for program counter
    pub program_counter_register : u32,

    // 16 bit register for memory index register
    pub index_register : u16,

    // 8 bit registers
    pub delay_timer_register : u8,
    pub sound_timer_register : u8,
    
    // flag registers
    pub vf_flag_register : bool,

    // 16 general purpose 8 bit registers 
    pub general_purpose_registers : Vec<u8>,
}

impl Chip8 {
    const SCREEN_HEIGHT : usize = 32;
    const SCREEN_WIDTH : usize = 64;

    pub fn new() -> Chip8 {
        Chip8 {
            memory : vec![0; 4096],
            display : vec![vec![true; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT],
            stack : vec![0; 16],
            program_counter_register : 0,
            index_register : 0,
            delay_timer_register : 0,
            sound_timer_register : 0,
            vf_flag_register : false,
            general_purpose_registers : vec![0; 16],
        }
    }

    pub fn decode(&mut self, instruction : u16) {
        const FXXX_BITMASK : u16 = 0xF000;
        const XFFF_BITMASK : u16 = 0x0FFF;
        const XFXX_BITMASK : u16 = 0x0F00;
        const XXFF_BITMASK : u16 = 0x00FF;

        match instruction {
            // 0x00E0 (clear screen) 
            0x00E0 => { self.clear_screen_instruction() }

            // 0x1NNN (jumps the program counter to a specific location)
            i if i & FXXX_BITMASK == 0x1000 => {
                // call the jump instruction with NNN from 1NNN to tell the program counter where to jump
                let location = i & XFFF_BITMASK;
                self.jump_instruction(location)
            }

            // 0x6XNN(set register VX)
            i if i & FXXX_BITMASK == 0x6000 => {
                //
                let register = (i & XFXX_BITMASK) >> 8;
                let number = i & XXFF_BITMASK;
                self.set_register_vx_instruction(register, number);
            }

            // add value to register vx0
            i if i & FXXX_BITMASK == 0x7000 => {
                //
                let register = (i & XFXX_BITMASK) >> 8;
                let number = i & XXFF_BITMASK;
                self.add_register_vx_instruction(register, number);
            }

            // set index register i
            i if i & FXXX_BITMASK == 0xA000 => {
                let number = i & XFFF_BITMASK;
                self.set_index_register_instruction(number)
            }
            
            // draw/display 
            //TODO FIX THIS
            i if i & FXXX_BITMASK == 0xD000 => { self.draw_display_instruction() }

            _ => {} 
        }
    }

    // this will set every byte storing info for the display to off
    fn clear_screen_instruction(&mut self) {
        self.display = vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT];
    }

    // this will just set the program counter to a specific location in program memory of NNN
    fn jump_instruction(&mut self, location : u16) {
        self.program_counter_register = location as u32
    }

    fn set_register_vx_instruction(&mut self, register : u16, number : u16) {
        self.general_purpose_registers[register as usize] = number as u8
    }

    fn add_register_vx_instruction(&mut self, register : u16, number : u16) {
        self.general_purpose_registers[register as usize] += number as u8
    }

    fn set_index_register_instruction(&mut self, number : u16) {
        self.index_register = number;
    }

    fn draw_display_instruction(&self) {
        // TODO : implement this later but i would update it to draw to an actual window
    }
}