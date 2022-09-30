use rip8::{ run_emulator };

fn main() {

    let args : Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("error : incorrect arguments required clockspeed (hz) and gamepath");
    }

    let parse_result = args[1].parse::<u32>();
    
    let hertz = match parse_result {
        Ok(parsed_number) => parsed_number,
        Err(_error) => panic!("error invalid argument given for clockspeed")
    };

    run_emulator(hertz, &args[2]);
}