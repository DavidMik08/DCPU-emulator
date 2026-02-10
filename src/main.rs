use std::env;
use std::fs::File;
use std::io::{self, Read, Error, BufReader, Write, Seek};



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
                println!("[INFO] Read: {c}");
            }
            Err(e) => {
                eprintln!("[ERROR] Reading byte: {e}");
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
    println!("[INFO] The lenght of the program in bytes is: {len}");
    len
}


fn get_inst(pc: &mut u32, ram: &mut Vec<u8>) -> Vec<u8> {
    let mut inst: Vec<u8> = Vec::new();
    for i in 0..4 {
        inst.push(ram[*pc as usize + i]);
    }
    inst
}

fn add_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u16 = in1 as u16 + in2 as u16;
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        if result > 255 {
            *carry_flag = true;
        } else {
            *carry_flag = false;
        }
    }
    
    let result: u8 = (result & 255) as u8;

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Added {in1} and {in2}, got {result}");

    result
}

fn sub_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: i16 = in1 as i16 - in2 as i16;
    
    if result < 0 {
        result *= -1;
        result &= 255;
        result = 255 - result;
        result += 1;
    }

    if *shift_right {
        result /= 2;
    }

    let result: u8 = result as u8;

    if !*ignore_flags {
        if in1 > in2 {
            *carry_flag = true;
        } else {
            *carry_flag = false;
        }
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Subtracted {in1} and {in2}, got {result}");

    result
}

fn or_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = in1 | in2;
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = false; 
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Or {in1} and {in2}, got {result}");

    result
}

fn nor_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = !(in1 | in2);
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }
    
    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else { 
            *zero_flag = false;
        }
    }

    println!("[INFO] Nor {in1} and {in2}, got {result}");

    result
}

fn and_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = in1 & in2;
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] And {in1} and {in2}, got {result}");

    result
}

fn nand_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = !(in1 & in2);
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Nand {in1} and {in2}, got {result}");

    result
}

fn xor_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = in1 ^ in2;
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Xor {in1} and {in2}, got {result}");

    result
}

fn xnor_inst(in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
let mut result: u8 = !(in1 ^ in2);
    
    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }

    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }

    println!("[INFO] Xnor {in1} and {in2}, got {result}");

    result
}

fn impl_inst (in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = in1 & !in2;

    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = false;
    }
    
    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }
    
    println!("[INFO] Impl {in1} and {in2}, got {result}");

    result
}

fn nimpl_inst (in1: u8, in2: u8, ignore_flags: &mut bool, shift_right: &mut bool, carry_flag: &mut bool, zero_flag: &mut bool) -> u8 {
    let mut result: u8 = !(in1 & !in2);

    if *shift_right {
        result /= 2;
    }

    if !*ignore_flags {
        *carry_flag = true;
    }
    
    if !*ignore_flags {
        if result == 0 {
            *zero_flag = true;
        } else {
            *zero_flag = false;
        }
    }
    
    println!("[INFO] Nimpl {in1} and {in2}, got {result}");

    result
}
fn biz(pc: &mut u32, zero_flag: &mut bool, addr: u32) -> bool {
    if *zero_flag {
        *pc = addr;
        println!("[INFO] Branched on zero to: {addr}");

        return true;
    }
    false
}

fn bnz(pc: &mut u32, zero_flag: &mut bool, addr: u32) -> bool {
    if !*zero_flag {
        *pc = addr;
        println!("[INFO] Branched on not zero to: {addr}");

        return true;
    }
    false
}

fn bic(pc: &mut u32, carry_flag: &mut bool, addr: u32) -> bool {
    if *carry_flag {
        *pc = addr;
        println!("[INFO] Branched on carry to: {addr}");

        return true;
    }
    false
}

fn bnc(pc: &mut u32, carry_flag: &mut bool, addr: u32) -> bool {
    if !*carry_flag {
        *pc = addr;
        println!("[INFO] Branched on not carry to: {addr}");

        return true;
    }
    false
}



fn emulate<R: std::io::Read>(count: u8, registers: &mut Vec<u8>, input_ports: &mut Vec<u8>, output_ports: &mut Vec<u8>, ram: &mut Vec<u8>, inst: &mut Vec<u8>, pc: &mut u32, zero_flag: &mut bool, carry_flag: &mut bool, input_reader: BufReader<R>, output_port_file: &mut File, port_clk_file: &mut File) -> io::Result<bool> {
    println!("");
    // Set clock to 00 at start of instruction
    port_clk_file.seek(io::SeekFrom::Start(0))?;
    port_clk_file.write_all(b"00")?;
    port_clk_file.flush()?;
    if count > 25 {
        return Ok(true);
    }
    *inst = get_inst(&mut *pc, &mut *ram);
    
    let input_port_data: Vec<char> = get_inputs(input_reader);
    
    for i in (0..8).step_by(2) {
        input_ports[i/2] = hex_to_dec(input_port_data[i+1], input_port_data[i]);
    }
    println!("[INFO] Program counter: {}", *pc);
    for i in 0..4 {
        println!("[INFO] {}", inst[i]);
    }
    let addr: u32 = (registers[2] as u32) + (registers[3] as u32) * 256 + (registers[4] as u32) * 65536;
    let mut sp: u32 = (registers[5] as u32) + (registers[6] as u32) * 256 + (registers[7] as u32) * 65536;

    if inst[0] & 128 != 128 {
        match inst[1] {
            0 => inst[1] = 0,
            1 => inst[1] = registers[0],
            2 => inst[1] = registers[1],
            3 => inst[1] = registers[2],
            4 => inst[1] = registers[3],
            5 => inst[1] = registers[4],
            6 => inst[1] = registers[5],
            7 => inst[1] = registers[6],
            8 => inst[1] = registers[7],


            9 => inst[1] = ram[addr as usize],
            10 => {
                inst[1] = ram[{sp += 1; sp as usize}];
                registers[5] = (sp & 0xFF)               as u8;
                registers[6] = ((sp & 0xFF00)   >> 0x8)  as u8;
                registers[7] = ((sp & 0xFF0000) >> 0x10) as u8;
            },
            11 => inst[1] = input_ports[0],
            12 => inst[1] = input_ports[1],
            13 => inst[1] = input_ports[2],
            14 => inst[1] = input_ports[3],
            _ => todo!(),
        }
    }
    if inst[0] & 64 !=  64{
        match inst[2] {
            0 => inst[2] = 0,
            1 => inst[2] = registers[0],
            2 => inst[2] = registers[1],
            3 => inst[2] = registers[2],
            4 => inst[2] = registers[3],
            5 => inst[2] = registers[4],
            6 => inst[2] = registers[5],
            7 => inst[2] = registers[6],
            8 => inst[2] = registers[7],

            9 => inst[2] = ram[addr as usize],
            10 => {
                inst[2] = ram[{sp += 1; sp as usize}];
                registers[5] = (sp & 0xFF)               as u8;
                registers[6] = ((sp & 0xFF00)   >> 0x8)  as u8;
                registers[7] = ((sp & 0xFF0000) >> 0x10) as u8;
            },

            11 => inst[2] = input_ports[0],
            12 => inst[2] = input_ports[1],
            13 => inst[2] = input_ports[2],
            14 => inst[2] = input_ports[3],
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
        6 => out = &mut registers[5],
        7 => out = &mut registers[6],
        8 => out = &mut registers[7],

        9 => out = &mut ram[addr as usize],
        10 => {
            out = &mut ram[{let tmp = sp; sp -= 1; tmp as usize}];
            registers[5] = (sp & 0xFF)               as u8;
            registers[6] = ((sp & 0xFF00)   >> 0x8)  as u8;
            registers[7] = ((sp & 0xFF0000) >> 0x10) as u8;
        },
        11 => out = &mut output_ports[0],
        12 => out = &mut output_ports[1],
        13 => out = &mut output_ports[2],
        14 => out = &mut output_ports[3],
        _ => todo!(),
    }
    println!("[INFO] Inputs are: {} and {}!!!", inst[1], inst[2]);
    let mut ignore_flags: bool = false;
    let mut shift_right: bool = false;
    if inst[0] & 32 == 32 {
        ignore_flags = true;
    }
    if inst[0] & 16 == 16 {
        shift_right = true;
    }
    let mut branched: bool = false;
    unsafe {
        match inst[0] & 15 {
            0  => *out = add_inst(  inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            1  => *out = sub_inst(  inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            2  => *out = or_inst(   inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            3  => *out = nor_inst(  inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            4  => *out = and_inst(  inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            5  => *out = nand_inst( inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            6  => *out = xor_inst(  inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            7  => *out = xnor_inst( inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            8  => *out = impl_inst( inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            9  => *out = nimpl_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag),
            10 => {
                branched = true;
                *pc = addr;
                *out = add_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag);
            }
            11 => {
                branched = biz(pc, zero_flag, addr);
                *out = add_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag);
            }
            12 => {
                branched = bnz(pc, zero_flag, addr);
                *out = add_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag);
            }
            13 => {
                branched = bic(pc, carry_flag, addr);
                *out = add_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag);
            }
            14 => {
                branched = bnc(pc, carry_flag, addr);
                *out = add_inst(inst[1], inst[2], &mut ignore_flags, &mut shift_right, carry_flag, zero_flag);
            }

            15 => return Ok(true),
            _ => todo!(),
        }
    }
    
    // Set clock to 01 after instruction execution
    port_clk_file.seek(io::SeekFrom::Start(0))?;
    port_clk_file.write_all(b"01")?;
    port_clk_file.flush()?;

    // Write output ports (seek to beginning to overwrite)
    output_port_file.seek(io::SeekFrom::Start(0))?;
    let formatted_output = format!("{:02x}{:02x}{:02x}{:02x}", output_ports[0], output_ports[1], output_ports[2], output_ports[3]);
    output_port_file.write_all(formatted_output.as_bytes())?;
    output_port_file.flush()?;

    if !branched {
    *pc += 4;
    }
    //dbg!(*pc);
    Ok(false)
}

fn get_inputs<R>(reader: io::BufReader<R>) -> Vec<char> where R: std::io::Read {
    let mut input: Vec<char> = Vec::new();

    // Read byte by byte and convert to char
    // NOTE: This only works for ASCII (0-127)
    for byte_result in reader.bytes() {
        match byte_result {
            Ok(byte) => {
                // Convert byte to char (ASCII only)
                let c: char = byte as char;
                input.push(c);
                println!("[INFO] Read input: {c}");
            }
            Err(e) => {
                eprintln!("[ERROR] Cant read input byte!!! \n{e}");
                break;
            }
        }
    }
    input
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err(Error::new(io::ErrorKind::InvalidInput, "[ERROR] Not enough arguments!"));
    }
    
    let file: File = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Cant open input file!!! \n{}", e);
            return Err(e);
        }
    };
    
    let mut output_port_file: File = match File::create("target/debug/ports/output_port.hex") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Cant create/open the output port file!!! \n{}", e);
            return Err(e);
        }
    };

    let mut port_clk_file: File = match File::create("target/debug/ports/clk.hex") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Cant create/open the port clock file!!! \n{}", e);
            return Err(e);
        }
    };

    let program: Vec<char> = get_program(io::BufReader::new(file));
    
    let mut registers: Vec<u8> = vec![0; 8];
    let mut input_ports: Vec<u8> = vec![0; 4];
    let mut output_ports: Vec<u8> = vec![0; 4];
    let mut ram: Vec<u8> = vec![0; 16777216];
    let mut inst: Vec<u8> = vec![0; 4];
    let mut pc: u32 = 0;
    let mut zero_flag: bool = false;
    let mut carry_flag: bool = false;

    let len: u32 = get_len(program.clone())*2;
    for i in (6..(len+6)).step_by(2) {
        
        let first_char = program[(i + 1) as usize];
        let second_char = program[i as usize];
        ram[(i/2 - 3) as usize] = hex_to_dec(first_char, second_char);
    }
    let mut count: u8 = 0;
    loop {
        // Reopen input port file each cycle to get fresh data
        let input_port_file = File::open("target/debug/ports/input_port.hex")?;
        
        if emulate(count, &mut registers, &mut input_ports, &mut output_ports, &mut ram, &mut inst, &mut pc, &mut zero_flag, &mut carry_flag, io::BufReader::new(input_port_file), &mut output_port_file, &mut port_clk_file)? {
            break;
        }
        count += 1;
    }

    println!("[INFO] Exited successfully!");
    Ok(())
}
