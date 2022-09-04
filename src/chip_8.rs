use std::{
    fs::File,
    io::BufRead,
    io::BufReader,
    thread,
    time,
};

// blog used : https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

pub struct Chip8 {
    // memory for the chip8 should be 4k
    // since I am using u16s for the instructions the size of the vector is 4096 / 2 = 2048
    // program memory should be full of nop statements by default wich are 0x0000
    pub memory : Vec<u16>,
    
    // the display is monochrome so the bytes representing pixels can just be bools
    // the display size should be 32 by 64
    pub display : Vec<Vec<bool>>,

    // the specs given don't say how many stack entries there should be but I put 16
    pub stack : Vec<u16>,
    
    // 32 bit register for program counter
    pub program_counter_register : u16,

    // 16 bit register for memory index register
    pub index_register : u16,

    // 8 bit registers
    pub delay_timer_register : u8,
    pub sound_timer_register : u8,
    
    // flag registers
    pub vf_flag_register : bool,
    pub jumped_flag_register : bool,

    // 16 general purpose 8 bit registers 
    pub general_purpose_registers : Vec<u8>,
}

impl Chip8 {
    const SCREEN_HEIGHT : usize = 32;
    const SCREEN_WIDTH : usize = 64;
    const PROGRAM_MEMORY_SIZE : usize = 2048;
    const STACK_SIZE : usize = 16;
    const CLOCK_SLEEP_TIME_SECONDS : f64 = 1.0 / 1_000_000.0;

    pub fn new() -> Chip8 {
        Chip8 {
            memory : vec![0; Self::PROGRAM_MEMORY_SIZE],
            display : vec![vec![true; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT],
            stack : vec![0; Self::STACK_SIZE],
            program_counter_register : 0,
            index_register : 0,
            delay_timer_register : 0,
            sound_timer_register : 0,
            vf_flag_register : false, 
            jumped_flag_register : false,
            general_purpose_registers : vec![0; 16],
        }
    }

    //TODO : ADD ERROR HANDLING
    pub fn load_rom(&mut self, file_path : &String) {
        let file_open_result = File::open(file_path);

        let file_handle = match file_open_result {
            Ok(file) => { file } 
            _ => { panic!("error : could not load file at specified path!") } 
        };

        let reader = BufReader::new(file_handle);
        for (index, line) in reader.lines().enumerate() {
            if index > Self::PROGRAM_MEMORY_SIZE - 1 {
                panic!("error : your program is too large! 4096 bytes of program memory maximum")
            }

            let line = line.unwrap();

            // figure out why this has to be 16 in the second arg
            let instruction_result = u16::from_str_radix(&line, 16);
            
            let instruction = match instruction_result {
                Ok(hex_number) => { hex_number }
                _ => { panic!("error : instructions must be in radix hex like 'XXXX' !") }
            };

            self.memory[index as usize] = instruction; 
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
            // if the program counter has run out of instructions break out of the processor loop
            if self.program_counter_register as usize > Self::PROGRAM_MEMORY_SIZE - 1 { break }

            // read in an instruction. this is basically the fetch step
            let instruction = self.memory[self.program_counter_register as usize];

            // decode and execute will both read the contents of the instruction and then execute the instruction afterwards
            self.decode_and_execute(instruction);

            // thise is here for debugging purposes to see what register 6 is doing
            println!("{}",self.general_purpose_registers[6]);

            // if there was a jump (or later on probably a call) dont increment the program counter
            if !self.jumped_flag_register {
                self.program_counter_register += 1;
            } else {
                self.jumped_flag_register = false;
            }

            thread::sleep(time::Duration::from_secs_f64(Self::CLOCK_SLEEP_TIME_SECONDS));
        }
    }

    /// this is a function that will debug and execute a single instruction
    /// 
    /// if the instruction does not match anything in the specified instruction list then it will act as a NOP
    pub fn decode_and_execute(&mut self, instruction : u16) {
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

            // TODO return
            0x00EE => {}

            // TODO call
            i if i & FXXX_BITMASK == 0x2000 => {}

            // TODO skip if vx is equal to nn
            i if i & FXXX_BITMASK == 0x3000 => {}

            // TODO skip if vx is not equal to nn
            i if i & FXXX_BITMASK == 0x4000 => {}

            // TODO skip if vx is not equal to nn
            i if i & FXXX_BITMASK == 0x4000 => {}

            // TODO skip if vx is not equal to nn
            i if i & FXXX_BITMASK == 0x5000 => {}

            _ => {
                print!("");
            }
        }
    }

    /// this will set every byte storing info for the display to off
    fn clear_screen_instruction(&mut self) {
        self.display = vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT];
    }

    /// this will just set the program counter to a specific location in program memory of NNN
    fn jump_instruction(&mut self, location : u16) {
        self.program_counter_register = location;
        self.jumped_flag_register = true
    }

    /// this will 
    fn set_register_vx_instruction(&mut self, register : u16, number : u16) {
        self.general_purpose_registers[register as usize] = number as u8
    }

    fn add_register_vx_instruction(&mut self, register : u16, number : u16) {
        // wrapping add will add and account for overflows
        self.general_purpose_registers[register as usize] = self.general_purpose_registers[register as usize].wrapping_add(number as u8)
    }

    fn set_index_register_instruction(&mut self, number : u16) {
        self.index_register = number;
    }

    fn draw_display_instruction(&self) {
        // TODO : implement this later but i would update it to draw to an actual window
    }
}