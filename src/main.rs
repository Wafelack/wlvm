use crate::Instructions::*;
use crate::Registers::*;
use std::io;
use std::io::Write;

mod parser;

const STACK_SIZE: usize = 255;

use parser::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instructions {
    Psh(i32),
    Add(Registers, Registers),
    Mul(Registers, Registers),
    Div(Registers, Registers),
    Sub(Registers, Registers),
    Pop,
    Mov(Registers, Registers),
    Hlt,
    Dst,
    Drg(Registers),
    Dmp,
    Prt(Registers), // Prints the ascii letter corresponding of the register's content
    Tee(Registers, Registers), // ==
    Tne(Registers, Registers), // !=
    Tll(Registers, Registers), // <
    Tmm(Registers, Registers), // >
    Tel(Registers, Registers), // <=
    Tem(Registers, Registers), // >=
    Jmp(i32),       // Jump to line if Eq is true
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Registers {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    Ip = 6,
    Sp = 7,
    St = 8,
    Eq = 9,
    NumOfRegisters = 10,
}

fn reg_name(reg: i32) -> &'static str {
    match reg {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "Ip",
        7 => "Sp",
        8 => "St",
        9 => "Eq",
        _ => "_ ",
    }
}

fn fetch(program: &Vec<Instructions>, ip: usize) -> Instructions {
    program[ip]
}

pub fn dump(stack: &Vec<i32>, regs: &[i32; NumOfRegisters as usize]) {
    print!("[");
    for i in 0..(NumOfRegisters as usize) {
        print!("{}: {}, ", reg_name(i as i32), regs[i]);
    }
    println!("]");
    println!();
    print!("Stack : [{}, ", stack[0]);
    for i in 1..stack.len() {
        if i == stack.len() - 1 {
            println!("{}]", stack[i]);
        } else {
            print!("{}, ", stack[i]);
        }
    }
}

fn eval(
    instr: Instructions,
    running: &mut bool,
    stack: &mut Vec<i32>,
    regs: &mut [i32; NumOfRegisters as usize],
    details: bool,
) {
    // Instrucion Pointer : regs[6]
    // Stack Pointer : regs[7]

    if details {
        print!("{} - ", regs[6]);
    }

    match instr {
        Dmp => dump(stack, regs),
        Prt(reg) => {
            if (0..256).contains(&regs[reg as usize]) {
                print!("{}", regs[reg as usize] as u8 as char);
                io::stdout().flush().unwrap();
            }
        }
        Tee(a, b) => {
            if details {
                println!("{} == {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] == regs[b as usize]) as i32;
        }
        Tne(a, b) => {
            if details {
                println!("{} != {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] != regs[b as usize]) as i32;
        }
        Tll(a, b) => {
            if details {
                println!("{} < {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] < regs[b as usize]) as i32;
        }
        Tmm(a, b) => {
            if details {
                println!("{} > {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] > regs[b as usize]) as i32;
        }
        Tel(a, b) => {
            if details {
                println!("{}  <= {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] <= regs[b as usize]) as i32;
        }
        Tem(a, b) => {
            if details {
                println!("{}  >= {}", regs[a as usize], regs[b as usize]);
            }
            regs[Eq as usize] = (regs[a as usize] >= regs[b as usize]) as i32;
        }
        Jmp(i) => {
            if i < 0 {
                panic!("ERR_ATEMPTED_TO_JUMP_TO_NEGATIVE_OPERATION_NUMBER");
            }
            if details {
                println!("Jumped to {}", i);
            }
            if regs[Eq as usize] == 1 {
                if details {
                    println!("Goto {}", i);
                }
                regs[Ip as usize] = i - 3;
            } else {
                if details {
                    println!("None");
                }
            }
        }
        Hlt => {
            if details {
                println!("Quit");
            }
            *running = false;
        }
        Psh(i) => {
            if (regs[7] + 1) as usize >= STACK_SIZE {
                panic!("ERR_STACK_OVERFLOW");
            }
            regs[7] += 1;
            stack[regs[7] as usize] = i;
            regs[8] = i;
            if details {
                println!("-> {}", i);
            }
        }
        Pop => {
            if regs[7] - 1 < 0 {
                panic!("ERR_STACK_UNDERFLOW");
            }
            let popped = stack[regs[7] as usize];
            regs[7] -= 1;
            regs[8] = stack[regs[7] as usize];
            if details {
                println!("<- {}", popped);
            }
        }
        Add(a, b) => {
            if details {
                println!("{} + {}", regs[a as usize], regs[b as usize]);
            }
            regs[a as usize] += regs[b as usize];
        }
        Sub(a, b) => {
            if details {
                println!("{} - {}", regs[a as usize], regs[b as usize]);
            }
            regs[a as usize] -= regs[b as usize];
        }
        Mul(a, b) => {
            if details {
                println!("{} * {}", regs[a as usize], regs[b as usize]);
            }
            regs[a as usize] *= regs[b as usize];
        }
        Div(a, b) => {
            if details {
                println!("{} / {}", regs[a as usize], regs[b as usize]);
            }
            regs[a as usize] /= regs[b as usize];
        }
        Mov(a, b) => {
            if details {
                println!("{} <-| {}", reg_name(a as i32), reg_name(b as i32));
            }
            if a == Ip {
                regs[a as usize] = regs[b as usize] - 3; // Being the same as jump
            }
            regs[a as usize] = regs[b as usize];
        }
        Drg(reg) => {
            println!("[{}]", regs[reg as usize]);
        }
        Dst => {
            for val in stack {
                if val != &0 {
                    println!("[{}]", val);
                }
            }
        }
    }
}

fn help() {
    println!(
        "wlvm version {} by Wafelack <wafelack@protonmail.com>\n",
        env!("CARGO_PKG_VERSION")
    );
    println!("usage: wlvm <command> [flags]\n");
    println!("COMMANDS:");
    println!("\trun <filename> : Runs the code file");
    println!("\tdump <filename>: Runs the program and dumps the memory");
    println!("\nFLAGS:");
    println!("\t--instructions | -d: Shows the instructions run in the program");
    println!("\t--details | -d     : Shows the details while running code");
    std::process::exit(0);
}

fn is_present(args: &Vec<String>, to_search: &str) -> bool {
    for arg in args {
        if arg == to_search {
            return true;
        }
    }
    false
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let mut program: Vec<Instructions> = vec![];

    let mut details = false;

    if args.len() < 1 {
        help();
    } else {
        if args[0] == "run" {
            if args.len() < 2 {
                help();
            } else {
                if !std::path::Path::new(&args[1]).exists() {
                    eprintln!("Error: no input files");
                    std::process::exit(66);
                } else {
                    program = parse_file(&args[1]);
                }
                if is_present(&args, "--instructions") || is_present(&args, "-i") {
                    println!("{:?}\n==============================", program);
                }
                if is_present(&args, "--details") || is_present(&args, "-d") {
                    details = true;
                }
            }
        } else if args[0] == "dump" {
            if args.len() < 2 {
                help();
            } else {
                if !std::path::Path::new(&args[1]).exists() {
                    eprintln!("Error: no input files");
                    std::process::exit(66);
                } else {
                    program = parse_file(&args[1]);
                    program.push(Dmp);
                    program = program
                        .iter()
                        .filter(|x| is_valid(**x))
                        .map(|s| *s)
                        .collect();
                    program.push(Dmp);
                    program.push(Hlt);
                }
            }
        } else {
            help();
        }
    }

    let mut running = true;
    let mut stack = vec![0; STACK_SIZE];
    let mut registers = [0; NumOfRegisters as usize];

    registers[Sp as usize] = -1;

    while running {
        let instr = fetch(&program, registers[6] as usize);
        eval(instr, &mut running, &mut stack, &mut registers, details);
        registers[6] += 1;
    }
}

fn is_valid(instr: Instructions) -> bool {
    match instr {
        Prt(_) => false,
        Drg(_) => false,
        Dst => false,
        Dmp => false,
        Hlt => false,
        _ => true,
    }
}
