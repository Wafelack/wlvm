use crate::Instructions::*;
use crate::Registers::*;
#[derive(Copy, Clone)]
enum Instructions {
    Psh(i32),
    Add,
    Pop,
    Set(Registers, i32),
    Hlt,
    ShowStack,
    ShowRegs,
}
#[derive(Copy, Clone)]
enum Registers {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    Ip = 6,
    Sp = 7,
    NumOfRegisters = 8
}

fn fetch(program: &Vec<Instructions>, regs: &mut [i32; NumOfRegisters as usize]) -> Instructions {
    program[regs[6] as usize]
}

fn eval(instr: Instructions, running: &mut bool, stack: &mut Vec<i32>, regs: &mut [i32; NumOfRegisters as usize]) {

    // Instrucion Pointer : regs[6]
    // Stack Pointer : regs[7]

    match instr {
        Hlt => *running = false,
        Psh(i) => {
            regs[7]+=1;
            stack[regs[7] as usize] = i;
            println!("-> {}", i);
        }
        Pop => {
            let popped = stack[regs[7] as usize];
            regs[7] -= 1;

            println!("<- {}", popped);
        }
        Add => {
            let a = stack[regs[7] as usize];
            regs[7] -= 1;
            let b = stack[regs[7] as usize];
            regs[7] -= 1;

            let result = b + a;
            regs[7] += 1;
            stack[regs[7] as usize] = result;

        }
        Set(reg,i) => {
            regs[reg as usize] = i;
        }
        ShowRegs => {
            println!("{:?}", regs);
        }
        ShowStack => {
            for val in stack {
                if val != &0 {
                    println!("[{}]", val);
                }
            }
        }
    }
}

fn main() {
    let program: Vec<Instructions> = vec![Psh(5), Psh(6), Add, Pop, Set(A, 45), ShowRegs, ShowStack, Hlt];

    let mut ip = 0usize;
    let mut running = true;
    let mut sp = -1;
    let mut stack = vec![0;256];
    let mut registers = [0; NumOfRegisters as usize];

    while running {

        let instr = fetch(&program, &mut registers);
        eval(instr,&mut running, &mut stack, &mut registers);
        registers[6] += 1;
    }
}
