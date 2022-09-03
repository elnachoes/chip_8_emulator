
// blog used : https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

pub struct Chip8 {
    // memory for the chip8 should be 4k
    pub memory : Vec<u8>,
    
    // the display is monochrome so the bytes representing pixels can just be bools
    // the display size should be 32 by 64
    pub display : Vec<Vec<bool>>,

    // the specs given don't say how many stack entries there should be but I put 16
    pub stack : Vec<u16>,
    
    // 16 bit registers for program counter and memory index register
    pub program_counter_register : u16,
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
    pub fn new() -> Chip8 {
        Chip8 {
            memory : vec![0; 4096],
            display : vec![vec![false; 32]; 64],
            stack : vec![0; 16],
            program_counter_register : 0,
            index_register : 0,
            delay_timer_register : 0,
            sound_timer_register : 0,
            vf_flag_register : false,
            general_purpose_registers : vec![0; 16],
        }
    }

    pub fn decode(&mut self, instruction : u32) {
        const LAST_3_NIBBLES_BITMASK : u32 = 0xF000;
        const FIRST_NIBBLE_BITMASK : u32 = 0x0FFF;

        match instruction {
            // 00E0 (clear screen) 
            0x00E0 => {
                print!("")
            }

            // jumps the program counter to a specific location
            i if i & LAST_3_NIBBLES_BITMASK == 0x1000 => {
                print!("")
            }

            // return
            0x00EE => {
                print!("")
            }

            // call
            i if i & LAST_3_NIBBLES_BITMASK == 0x2000 => {}

            // call
            i if i & LAST_3_NIBBLES_BITMASK == 0x3000 => {}


            _ => {} 
        }
    }
}