use std::convert::TryInto;
use std::io::{self, Read};

pub fn intcode(code: Vec<i64>, pc: usize) -> Vec<i64> {
  let inputs = Vec::<i64>::new();
  intcodes(code, pc, inputs)
}

pub fn intcodes(code: Vec<i64>, pc: usize, inputs: Vec<i64>) -> Vec<i64> {
  intcodesw(code, pc, inputs, &mut io::stdout())
}

pub fn intcodesw(
      code: Vec<i64>, 
      pc: usize, 
      inputs: Vec<i64>,
      stdout: &mut io::Write) 
    -> Vec<i64>
{
  let mut reversed_inputs = inputs.clone();
  reversed_inputs.reverse();
  intcodes_internal(code, pc, &mut reversed_inputs, stdout)
}

fn intcodes_internal(
      code: Vec<i64>, 
      pc: usize, 
      inputs: &mut Vec<i64>,
      stdout: &mut io::Write) 
    -> Vec<i64>
{
  // Intcode
  // in set, place 0: opcode
  // opcode 99: halt command processing
  // opcode 1 (add): vec[set[3]] = vec[set[1]] + vec[set[2]]
  // opcode 2 (mul): vec[set[3]] = vec[set[1]] * vec[set[2]]

  let mut outputs = code.clone();
  let op:i64 = code[pc];

  //println!("DEBUG:Intcode:: op:{}", op);
  if op == 99 {
    writeln!(stdout, "Intcode:: EXIT (SUCCESS)").ok();
    return outputs;
  }
  if op < 1 || op > 1198 {
    writeln!(stdout, "Intcode:: Received EXIT (FAILURE)").ok();
    return outputs;
  }

  let parameterized = |x: i64, op: i64| {
    op == x 
    || op == (x + 100)
    || op == (x + 1000)
    || op == (x + 1100)
  };

  let op_add: bool = parameterized(1, op);
  let op_mul: bool = parameterized(2, op);
  let op_input: bool = op == 3;
  let op_output: bool = op == 4 || op == 104;
  let op_jump_true: bool = parameterized(5, op);
  let op_jump_false: bool = parameterized(6, op);
  let op_less_than: bool = parameterized(7, op);
  let op_equals: bool = parameterized(8, op);

  if op_input {
    // input
    let input: i64;
    if inputs.len() == 0 {
      writeln!(stdout, "Intcode:: Input: ").ok();
      let mut buffer = String::new();
      io::stdin().read_to_string(&mut buffer).ok();
      input = buffer.trim().parse().unwrap();
    } else {
      input = inputs.pop().unwrap();
    }
    //let dest: usize = inputs[pc + 3]
    let dest: usize = code[pc + 1]
      .try_into()
      .unwrap();
    outputs[dest] = input;
    return intcodes_internal(outputs, pc + 2, inputs, stdout);
  }

  // get the a parameter, handling parameter/immediate mode
  let mut a: i64 = code[pc + 1];
  if op < 10 || (op > 1000 && op < 1100) {
    a = *code.get(a as usize).unwrap() as i64;
  }

  if op_output {
    // output
    writeln!(stdout, "Intcode:: Print: {}", a).ok();
    return intcodes_internal(outputs, pc + 2, inputs, stdout);
  } 

  // get the b parameter, handling parameter/immediate mode
  let mut b: i64 = code[pc + 2];
  if op < 10 || (op > 100 && op < 1000) {
    b = *code.get(b as usize).unwrap() as i64;
  }

  if op_jump_true || op_jump_false {
    // jump-if-true | jump-if-false
    // 0n->paramAB, 10n->paramB, 100n->paramA, 110n->immediate
    let mut new_pc: usize = pc + 3;
    if (a != 0 && op_jump_true) || (a == 0 && op_jump_false) {
      new_pc = b as usize;
    }
    return intcodes_internal(outputs, new_pc, inputs, stdout);
  }

  // Get the third operation
  let dest: usize = code[pc + 3]
    .try_into()
    .unwrap();

  if op_less_than || op_equals {
    if (op_less_than && a < b) || (op_equals && a == b) {
      outputs[dest] = 1;
    } else {
      outputs[dest] = 0;
    }
    return intcodes_internal(outputs, pc + 4, inputs, stdout);
  }
  else if op_add || op_mul {
    // add and multiply
    if op_add {
      outputs[dest] = a + b;
    } else {
      outputs[dest] = a * b;
    }
    return intcodes_internal(outputs, pc + 4, inputs, stdout);
  }
  writeln!(stdout, "Intcode:: Received EXIT (FAILURE)").ok();
  outputs
}
