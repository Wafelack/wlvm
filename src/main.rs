use crate::Instructions::*;
use crate::Registers::*;

mod parser;

use parser::*;
#[derive(Copy, Clone, Debug)]
pub enum Instructions {
    Psh(i32),
    Add(Registers, Registers),
    Mul(Registers, Registers),
    Div(Registers, Registers),
    Sub(Registers, Registers),
    Pop,
    Set(Registers, i32),
    Mov(Registers, Registers),
    Hlt,
    Dst,
    Drg(Registers),
    Peek,
    Tee(Registers, Registers), // ==
    Tne(Registers, Registers), // !=
    Tll(Registers, Registers), // <
    Tmm(Registers, Registers), // >
    Tel(Registers, Registers), // <=
    Tem(Registers, Registers), // >=
    Jmp(i32), // Jump to line if Eq is true
}
#[derive(Copy, Clone, Debug)]
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
    NumOfRegisters = 10
}

fn fetch(program: &Vec<Instructions>, ip: usize) -> Instructions {
    program[ip]
}

fn eval(instr: Instructions, running: &mut bool, stack: &mut Vec<i32>, regs: &mut [i32; NumOfRegisters as usize], details: bool) {

    // Instrucion Pointer : regs[6]
    // Stack Pointer : regs[7]

    match instr {
        Tee(a, b) => {
            regs[Eq as usize] = (regs[a as usize] == regs[b as usize]) as i32;
        }
        Tne(a, b) => {
            regs[Eq as usize] = (regs[a as usize] != regs[b as usize]) as i32;
        }
        Tll(a, b) => {
            regs[Eq as usize] = (regs[a as usize] < regs[b as usize]) as i32;
        }
        Tmm(a, b) => {
            regs[Eq as usize] = (regs[a as usize] > regs[b as usize]) as i32;
        }
        Tel(a, b) => {
            regs[Eq as usize] = (regs[a as usize] <= regs[b as usize]) as i32;
        }
        Tem(a, b) => {
            regs[Eq as usize] = (regs[a as usize] >= regs[b as usize]) as i32;
        }
        Jmp(i) => {
            if regs[Eq as usize] == 1 {
                regs[Ip as usize] = i;
            }
        }
        Hlt => *running = false,
        Psh(i) => {
            regs[7]+=1;
            stack[regs[7] as usize] = i;
            regs[8] = i;
            if details {
                println!("-> {}", i);
            }
        }
        Pop => {
            let popped = stack[regs[7] as usize];
            regs[7] -= 1;
            regs[8] = stack[regs[7] as usize];
            if details {
                println!("<- {}", popped);
            }
        }
        Add(a, b) => {
            regs[a as usize] += regs[b as usize];
        }
        Sub(a, b) => {
            regs[a as usize] -= regs[b as usize];
        }
        Mul(a, b) => {
            regs[a as usize] *= regs[b as usize];
        }
        Div(a,b) => {
            regs[a as usize] /= regs[b as usize];
        }
        Set(reg,i) => {
            if let Ip = reg{
                regs[reg as usize] = i - 1;
            } else {
                regs[reg as usize] = i;
            }
        }
        Mov(a, b) => {
            regs[a as usize] = regs[b as usize];
        }
        Drg(reg) => {
            println!("[{}]", regs[reg as usize]);
        }
        Peek => {
            println!(" # {}", stack[regs[7] as usize]);
            regs[8] = stack[regs[7] as usize];
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
    println!("wlvm version 0.2.0 by Wafelack <wafelack@protonmail.com>\n");
    println!("usage: wlvm <command> [flags]\n");
    println!("COMMANDS:");
    println!("\trun <filename> : Runs the code file");
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

    let mut program : Vec<Instructions> = vec![];

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
            }

        } else {
            help();
        }
    }

    let mut running = true;
    let mut stack = vec![0;256];
    let mut registers = [0; NumOfRegisters as usize];

    while running {
        let instr = fetch(&program, registers[6] as usize);
        eval(instr,&mut running, &mut stack, &mut registers, is_present(&args, "--details") || is_present(&args, "-d"));
        registers[6] += 1;
    }
}
