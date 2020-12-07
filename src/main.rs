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
    NumOfRegisters = 9
}

fn fetch(program: &Vec<Instructions>, ip: usize) -> Instructions {
    program[ip]
}

fn eval(instr: Instructions, running: &mut bool, stack: &mut Vec<i32>, regs: &mut [i32; NumOfRegisters as usize]) {

    // Instrucion Pointer : regs[6]
    // Stack Pointer : regs[7]

    match instr {
        Hlt => *running = false,
        Psh(i) => {
            regs[7]+=1;
            stack[regs[7] as usize] = i;
            regs[8] = i;
            println!("-> {}", i);
        }
        Pop => {
            let popped = stack[regs[7] as usize];
            regs[7] -= 1;
            regs[8] = stack[regs[7] as usize];

            println!("<- {}", popped);
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

fn main() {
    let program: Vec<Instructions> = parse_file("example.vm");

    let mut running = true;
    let mut stack = vec![0;256];
    let mut registers = [0; NumOfRegisters as usize];

    println!("{:?}", program);

    while running {
        let instr = fetch(&program, registers[6] as usize);
        eval(instr,&mut running, &mut stack, &mut registers);
        registers[6] += 1;
    }
}
