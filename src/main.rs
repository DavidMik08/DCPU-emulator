use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Error};



fn hex_to_dec(first_char: char, second_char: char) -> u8 {
    let mut result: u8 = 0;

    if first_char as u8 - 48 < 10 {
        result += first_char as u8 - 48;
    }
    else {
        match first_char {
            'a' | 'A' => result += 10,
            'b' | 'B' => result += 11,
            'c' | 'C' => result += 12,
            'd' | 'D' => result += 13,
            'e' | 'E' => result += 14,
            'f' | 'F' => result += 15,
            _ => todo!(),
        }
    }

    if second_char as u8 - 48 < 10 {
        result += (second_char as u8 - 48) * 16;
    }
    else {
        match second_char {
            'a' | 'A' => result += 10 * 16,
            'b' | 'B' => result += 11 * 16,
            'c' | 'C' => result += 12 * 16,
            'd' | 'D' => result += 13 * 16,
            'e' | 'E' => result += 14 * 16,
            'f' | 'F' => result += 15 * 16,
            _ => todo!(),
        }
    }

    result
}


fn get_program<R>(reader: io::BufReader<R>) -> Vec<char> where R: std::io::Read {
    let mut program: Vec<char> = Vec::new();

    // Read byte by byte and convert to char
    // NOTE: This only works for ASCII (0-127)
    for byte_result in reader.bytes() {
        match byte_result {
            Ok(byte) => {
                // Convert byte to char (ASCII only)
                let c: char = byte as char;
                program.push(c);
            }
            Err(e) => {
                eprintln!("Error reading byte: {}", e);
                break;
            }
        }
    }
    program
}


fn get_len(program: Vec<char>) -> u32 {
    let mut len: u32 = 0;
    for i in (0..6).step_by(2) {
        let byte: u8 = hex_to_dec(program[i+1], program[i]);
        len+=byte as u32 * i32::pow(16, 2-(i as u32)/2) as u32;
    }
    dbg!(len);
    len
}


fn get_inst(pc: &mut u32, ram: &mut Vec<u8>) -> Vec<u8> {
    let mut inst: Vec<u8> = Vec::new();
    for i in 0..4 {
        inst.push(ram[*pc as usize + i]);
    }
    inst
}

fn add(in1: u8, in2: u8, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let result: u16 = (in1 + in2).into();
    
    if result > 255 {
        *carry_flag = true;
    } else {
        *carry_flag = false;
    }
    
    let result: u8 = result as u8;

    if result == 0 {
        *zero_flag = true;
    } else {
        *zero_flag = false;
    }

    result
}

fn sub(in1: u8, in2: u8, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let result: u16 = (in1 - in2).into();

    if result > 255 {
        *carry_flag = true;
    } else {
        *carry_flag = false;
    }

    let result: u8 = result as u8;

    if result == 0 {
        *zero_flag = true;
    } else {
        *zero_flag = false;
    }

    result
}


fn emulate(registers: &mut Vec<u8>, ram: &mut Vec<u8>, stk: &mut Vec<u8>, inst: &mut Vec<u8>, pc: &mut u32, sp: &mut u8, zero_flag: &mut bool, carry_flag: &mut bool) -> bool {
    *inst = get_inst(&mut *pc, &mut *ram);
    let addr: u32 = (registers[2] as u32) + (registers[3] as u32) * 256 + (registers[4] as u32) * 65536;
    for i in &mut *inst {
        println!("Instruction part {}", i);
    }
    if inst[0] & 128 != 128 {
        match inst[1] {
            0 => inst[1] = 0,
            1 => inst[1] = registers[0],
            2 => inst[1] = registers[1],
            3 => inst[1] = registers[2],
            4 => inst[1] = registers[3],
            5 => inst[1] = registers[4],

            6 => inst[1] = ram[addr as usize],
            7 => inst[1] = stk[{let tmp = *sp; *sp -= 1; tmp as usize}],
            _ => todo!(),
        }
    }
    let out: *mut u8;
    let mut reg0: u8 = 0;

    match inst[3] {
        0 => out = &mut reg0,
        1 => out = &mut registers[0],
        2 => out = &mut registers[1],
        3 => out = &mut registers[2],
        4 => out = &mut registers[3],
        5 => out = &mut registers[4],

        6 => out = &mut ram[addr as usize],
        7 => out = &mut stk[{*sp += 1; *sp as usize}],
        _ => todo!(),
    }
    
    unsafe {
        match inst[0]&63 {
            0  => *out = add(inst[1], inst[2], carry_flag, zero_flag),
            1  => *out = sub(inst[1], inst[2], carry_flag, zero_flag),
            63 => return true,
            _ => todo!(),
        }
    }
    *pc += 4;
    dbg!(*pc);
    false
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err(Error::new(io::ErrorKind::InvalidInput, "ERROR: Not enough arguments!"));
    }
    
    let file: File = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };
   
    let program: Vec<char> = get_program(io::BufReader::new(file));
    
    let mut registers: Vec<u8> = vec![0; 5];
    let mut ram: Vec<u8> = vec![0; 16777216];
    let mut stk: Vec<u8> = vec![0; 256];
    let mut inst: Vec<u8> = vec![0; 4];
    let mut pc: u32 = 0;
    let mut sp: u8 = 0;
    let mut zero_flag: bool = false;
    let mut carry_flag: bool = false;

    let len: u32 = get_len(program.clone());
    for i in (6..program.len()).step_by(2) {
        // Ensure we don't go out of bounds
        if i + 1 < program.len() {
            let first_char = program[i + 1];
            let second_char = program[i];
            ram[i/2-3] = hex_to_dec(first_char, second_char);
            println!("RAM: {}: {}", i/2-3, ram[i/2-3]);
        }
    }

    loop {
        if emulate(&mut registers, &mut ram, &mut stk, &mut inst, &mut pc, &mut sp, &mut zero_flag, &mut carry_flag) {
            break Ok(());
        }
    }
}
