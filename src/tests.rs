use crate::{Instruction::*, Register::*, *};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn validation() {
    assert!(!is_valid(Prt(A)));
    assert!(!is_valid(Drg(A)));
    assert!(!is_valid(Dmp));
    assert!(!is_valid(Hlt));
  }

  #[test]
  fn stack() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();
    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(stack[0], 5);
    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(stack[1], 8);
    eval(
      &labels,
      Pop,
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Pop,
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(14),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(stack[0], 14);
  }

  #[test]
  fn registers_moving() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[A as usize], 5);
    eval(
      &labels,
      Mov(B, A),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[B as usize], 5);
  }

  #[test]
  fn registers_add() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Add(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[A as usize], 11);
  }

  #[test]
  fn registers_sub() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Sub(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[A as usize], -1);
  }

  #[test]
  fn registers_mul() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mul(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[A as usize], 30);
  }

  #[test]
  fn registers_div() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(10),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Div(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[A as usize], 2);
  }

  #[test]
  fn halt_program() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(10),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Div(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Hlt,
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert!(!running);
  }

  #[test]
  fn equality() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tee(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }
  #[test]
  fn non_equality() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tne(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }

  #[test]
  fn lower_than() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(5),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tll(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }

  #[test]
  fn greater_than() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(6),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tmm(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }
  #[test]
  fn greater_or_equal() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tem(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }

  #[test]
  fn lower_or_equal() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tel(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    assert_eq!(registers[Eq as usize], 1);
  }

  #[test]
  fn jump() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let labels: BTreeMap<String, i32> = BTreeMap::new();

    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(A, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Psh(8),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    eval(
      &labels,
      Mov(B, St),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Tel(A, B),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );

    eval(
      &labels,
      Jmp(4),
      &mut running,
      &mut stack,
      &mut registers,
      false,
    );
    assert_eq!(registers[Ip as usize], 2);
  }
  #[test]
  fn labels() {
    let (mut stack, mut registers, mut running) = setup_environment();

    let (program, labels) = parse_code(
      "psh 4\nmov a st\npsh 7\nmov b st\ngto :avoid_adding\nadd a b\n:avoid_adding\nhlt",
    );

    while running {
      let instr = fetch(&program, registers[6] as usize);
      eval(
        &labels,
        instr,
        &mut running,
        &mut stack,
        &mut registers,
        false,
      );

      registers[6] += 1;
    }

    assert_eq!(registers[A as usize], 4);
  }
}
