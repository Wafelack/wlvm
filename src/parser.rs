use crate::{Instructions, Instructions::*, Registers::*};
use std::fs;

fn error(line: usize, whr: &str, message: &str) {
  eprintln!("{} | {}", line, whr);
  eprintln!("^^^^^^^^^^^^^^^^^^^^");
  eprintln!("{}", message);
}

pub fn parse_file(filename: &str) -> Vec<Instructions> {

  let mut instrs: Vec<Instructions> = vec![];

  let fc = match fs::read_to_string(filename) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Failed to read file");
      eprintln!("{}", e);
      std::process::exit(-1);
    }
  };
  let lines = fc.split('\n').collect::<Vec<&str>>();

  let mut ln = 0usize;
  for line in lines {
    ln += 1;
    let splited = line.split(' ').collect::<Vec<&str>>();

    if line.starts_with(";") { continue; }

    match splited[0] {
      "psh" => {
          if splited.len() < 2 {
            error(ln, line, "Syntax error: valid syntax: `psh <integer>`");
            std::process::exit(-8);
          }

          let to_psh = match  splited[1].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => {
              error(ln, line, &format!("Type error : {} is not a valid integer", splited[1]));
              std::process::exit(-7);
            }
          };

          instrs.push(Psh(to_psh));
      }
      "mov" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `mov <register_a> <register_b>`");
          std::process::exit(-6);
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_a));
          std::process::exit(-5);
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_b));
          std::process::exit(-5);
        };

        instrs.push(Mov(reg_a, reg_b));

      }
      "set" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `set <register_a> <integer>`");
          std::process::exit(-6);
        }

        let raw = splited[1];

        let reg = if raw == "a" {
          A
        } else if raw == "b" {
          B
        } else if raw == "c" {
          C
        } else if raw == "e" {
          D
        } else if raw == "d" {
          E
        } else if raw == "f" {
          F
        } else if raw == "ip" {
          Ip
        } else if raw == "sp" {
          Sp
        } else if raw == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw));
          std::process::exit(-5);
        };

        let val = match splited[2].parse::<i32>() {
          Ok(i) => i,
          Err(_e) => {
            error(ln, line, &format!("Type error : {} is not a valid integer", splited[2]));
            std::process::exit(-7);
          }
        };

        instrs.push(Set(reg, val));

      }
      "add" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `mov <register_a> <register_b>`");
          std::process::exit(-6);
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_a));
          std::process::exit(-5);
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_b));
          std::process::exit(-5);
        };

        instrs.push(Add(reg_a, reg_b));

      }
      "sub" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `mov <register_a> <register_b>`");
          std::process::exit(-6);
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_a));
          std::process::exit(-5);
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_b));
          std::process::exit(-5);
        };

        instrs.push(Sub(reg_a, reg_b));

      }
      "mul" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `mov <register_a> <register_b>`");
          std::process::exit(-6);
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_a));
          std::process::exit(-5);
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_b));
          std::process::exit(-5);
        };

        instrs.push(Mul(reg_a, reg_b));

      }
      "div" => {
        if splited.len() < 3 {
          error(ln, line, "Syntax error: valid syntax: `mov <register_a> <register_b>`");
          std::process::exit(-6);
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_a));
          std::process::exit(-5);
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw_b));
          std::process::exit(-5);
        };

        instrs.push(Div(reg_a, reg_b));

      }
      "pop" => instrs.push(Pop),
      "dst" => instrs.push(Dst),
      "peek" => instrs.push(Peek),
      "drg" => {
        if splited.len() < 2 {
          error(ln, line, "Syntax error: valid syntax: `drg <register>`");
          std::process::exit(-6);
        }

        let raw = splited[1];

        let reg = if raw == "a" {
          A
        } else if raw == "b" {
          B
        } else if raw == "c" {
          C
        } else if raw == "e" {
          D
        } else if raw == "d" {
          E
        } else if raw == "f" {
          F
        } else if raw == "ip" {
          Ip
        } else if raw == "sp" {
          Sp
        } else if raw == "st" {
          St
        } else {
          error(ln, line, &format!("Type error : {} is not a valid register", raw));
          std::process::exit(-5);
        };

        instrs.push(Drg(reg));

      }
      "hlt" => {
        instrs.push(Hlt);
        return instrs;
      }

      _ => (),
    }

  }
  instrs.push(Hlt);
  instrs
}