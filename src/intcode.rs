use std::convert::TryInto;
use std::io::{self, Read};
use std::collections::HashMap;

pub fn int_input(in_str: &str) -> Vec<i64> {
  //println!("in: {}", &in_str.trim());
  let results = in_str
    .trim()
    .split(',')
    .filter_map(|x| if x.trim() != "" { x.trim().parse::<i64>().ok() } else { None })
    .collect();
  //println!("got input");
  results
}

pub fn intcode(code: Vec<i64>, pc: usize) -> (Vec<i64>, Vec<i64>) {
  let inputs = Vec::<i64>::new();
  intcodes(code, pc, inputs)
}

pub fn intcodes(code: Vec<i64>, pc: usize, inputs: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
  intcodesw(code, pc, inputs, &mut io::stdout())
}

pub fn intcodesq(
      code: Vec<i64>, 
      pc: usize, 
      inputs: Vec<i64>) 
    -> (Vec<i64>, Vec<i64>)
{
  intcodesw(code, pc, inputs, &mut io::sink())
}

pub fn intcodesw(
      code: Vec<i64>, 
      pc: usize, 
      inputs: Vec<i64>,
      stdout: &mut io::Write) 
    -> (Vec<i64>, Vec<i64>)
{
  let mut reversed_inputs = inputs.clone();
  reversed_inputs.reverse();
  let mut signals = Vec::<i64>::new();
  let mut base = 0;
  let memory: HashMap<i64, i64> = HashMap::new();
  intcodes_internal(code, pc, &mut reversed_inputs, false, stdout,
    &mut base, &mut signals, memory)
}

pub fn intcodesf(
      code: Vec<i64>, 
      pc: usize, 
      inputs: Vec<i64>,
      mut signals: &mut Vec<i64>) 
    -> (Vec<i64>, Vec<i64>)
{
  let mut reversed_inputs = inputs.clone();
  reversed_inputs.reverse();
  let mut base = 0;
  let memory: HashMap<i64, i64> = HashMap::new();
  intcodes_internal(code, pc, &mut reversed_inputs, true, &mut io::sink(),
    &mut base, &mut signals, memory)
  //intcodes_internal(code, pc, &mut reversed_inputs, true, &mut io::stdout(), &mut signals)
}

fn update_dest(input: i64, dest: i64, mut code: Vec<i64>, mut memory: HashMap<i64, i64>) -> (Vec<i64>, HashMap<i64, i64>) {
  if dest < (code.len() as i64) {
    //println!("update: address: {} code.len:{} memory.len:{} = {} in code", dest, code.len(), memory.len(), input);
    code[dest as usize] = input;
  } else {
    //println!("update: address: {} code.len:{} memory.len:{} = {} in memory", dest, code.len(), memory.len(), input);
    memory.insert(dest, input);
  }
  (code, memory)
}

fn lookup_memory(pc: i64, code: &Vec<i64>, memory: &HashMap<i64, i64>) -> i64 {
  if (pc as usize) < code.len() {
    //println!("lookup: address: {} code.len:{} memory.len:{} in code", pc, code.len(), memory.len());
    return *code.get(pc as usize).unwrap() as i64;
  } else {
    //println!("lookup: address: {} code.len:{} memory.len:{} in memory", pc, code.len(), memory.len());
    let mut new_val: i64 = 0;
    if memory.get(&pc).is_some() {
      new_val = *memory.get(&pc).unwrap() as i64;
    }
    return new_val;
  }
}

fn intcodes_internal(
      code: Vec<i64>, 
      pc: usize, 
      inputs: &mut Vec<i64>,
      feedback: bool,
      stdout: &mut io::Write,
      mut rel_base: &mut i64,
      mut signals: &mut Vec<i64>,
      memory: HashMap<i64, i64>) 
    -> (Vec<i64>, Vec<i64>)
{
  // Intcode
  // in set, place 0: opcode
  // opcode 99: halt command processing
  // opcode 1 (add): vec[set[3]] = vec[set[1]] + vec[set[2]]
  // opcode 2 (mul): vec[set[3]] = vec[set[1]] * vec[set[2]]

  let outputs = code.clone();
  //let code_outputs = Vec::<i64>::new();
  let op:i64 = code[pc];

  //println!("DEBUG:Intcode:: op:{}", op);
  if op == 99 {
    writeln!(stdout, "Intcode:: EXIT (SUCCESS)").ok();
    return (outputs, signals.to_vec());
  }
  if op < 1 || op > 22298 {
    writeln!(stdout, "Intcode:: Received EXIT (FAILURE)").ok();
    println!("EXIT badop: op: {} lengths:: code:{} memory:{} signals:{}", op, code.len(), memory.len(), signals.len());
    signals.push(i64::min_value());
    return (outputs, signals.to_vec());
  }

  let parameterized = |x: i64, op: i64| {
    // 0 = position, 1 = immediate, 2 = relative base
    op == x 
    || op == (x +   100)  /* a is immediate, b is position */
    || op == (x +   200)  /* a is relative,  b is position */
    || op == (x +  1000) /* a is position,  b is immediate */
    || op == (x +  2000) /* a is position,  b is relative */
    || op == (x +  1100) /* a is immediate, b is immediate */
    || op == (x +  1200) /* a is relative,  b is immediate */
    || op == (x +  2100) /* a is immediate, b is relative */
    || op == (x +  2200) /* a is relative,  b is relative */
    || op == (x + 20000) /* dest is relative */
    || op == (x + 20100) /* dest is relative, a immediate */
    || op == (x + 21100) /* dest is relative, a immediate, b immediate */
    || op == (x + 21200) /* dest is relative, a relative, b immediate */
    || op == (x + 22100) /* dest is relative, a immediate, b relative */
    || op == (x + 22200) /* dest is relative, a relative, b relative */
  };
  let modes = |x: i64| -> (u8, u8, u8) {
    // 0 = position, 1 = immediate, 2 = relative base
    // 100s  -> update a
    // 1000s -> update b
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    let mut d: u8 = 0;
    if x < 100 /* position for all */ {
      return (a, b, d);
    }
    a = ((x / 100) % 10).try_into().unwrap();
    if x >= 1000 {
      b = ((x / 1000) % 10).try_into().unwrap();
    }
    if x >= 10000 {
      d = ((x / 10000) % 10).try_into().unwrap();
    }
    (a, b, d)
  };

  let mode = modes(op);
  let op_add: bool = parameterized(1, op);
  let op_mul: bool = parameterized(2, op);
  let op_input: bool = op == 3 || op == 203;
  let op_output: bool = op == 4 || op == 104 || op == 204;
  let op_jump_true: bool = parameterized(5, op); // no dest
  let op_jump_false: bool = parameterized(6, op); // no dest
  let op_less_than: bool = parameterized(7, op);
  let op_equals: bool = parameterized(8, op);
  let op_rel: bool = op == 9 || op == 109 || op == 209;

  // get the a parameter, handling mode
  let mut a: i64 = code[pc + 1];
  let mut dest: i64 = a;
  if mode.0 == 0 {
    //a = *code.get(a as usize).unwrap() as i64;
    a = lookup_memory(a, &code, &memory);
  } else if mode.0 == 2 {
    //a = *code.get((a + *rel_base) as usize).unwrap() as i64;
    dest = a + *rel_base;
    a = lookup_memory(dest, &code, &memory);
  }
  //println!("DEBUG:: Looked up: a [{}] = {}", code[pc + 1], a);

  if op_input {
    //println!("DEBUG:: input  <{}:{} ({})> +{} mode({}, {}, {})", op, dest, code[pc + 1], rel_base, mode.0, mode.1, mode.2);
    // input
    let input: i64;
    if inputs.len() > 0 {
      input = inputs.pop().unwrap();
      writeln!(stdout, "Intcode:: Input: {}", input).ok();
    }
    else if feedback && signals.len() > 0 {
      input = signals.remove(0);
      writeln!(stdout, "Intcode:: Input: {}", input).ok();
    }
    else {
      writeln!(stdout, "Intcode:: Input: ").ok();
      let mut buffer = String::new();
      io::stdin().read_to_string(&mut buffer).ok();
      input = buffer.trim().parse().unwrap();
    }
    let (outputs, memory) = update_dest(input, dest, outputs, memory);
    return intcodes_internal(outputs, pc + 2, inputs, feedback, stdout, &mut rel_base, &mut signals, memory)
  }
  else if op_output {
    //println!("DEBUG:: output <{}:{}> +{} mode({}, {}, {})", op, a, rel_base, mode.0, mode.1, mode.2);
    // output
    writeln!(stdout, "Intcode:: Print: {}", a).ok();
    signals.push(a);
    let (outputs, _) = intcodes_internal(outputs, pc + 2, inputs, feedback, stdout, &mut rel_base, &mut signals, memory);
    return (outputs, signals.to_vec());
  } 
  else if op_rel {
    //println!("DEBUG:: +rel   <{}:{} ({})> +{} mode({}, {}, {})", op, a, code[pc + 1], rel_base, mode.0, mode.1, mode.2);
    *rel_base += a;
    return intcodes_internal(outputs, pc + 2, inputs, feedback, stdout, &mut rel_base, &mut signals, memory);
  }

  // get the b parameter, handling mode
  let mut b: i64 = code[pc + 2];
  if mode.1 == 0 {
    //b = *code.get(b as usize).unwrap() as i64;
    b = lookup_memory(b, &code, &memory);
  } else if mode.1 == 2 {
    //b = *code.get((b + *rel_base) as usize).unwrap() as i64;
    b = lookup_memory(b + *rel_base, &code, &memory);
  }
  //println!("DEBUG:: Looked up: b [{}] = {}", code[pc + 2], b);

  if op_jump_true || op_jump_false {
    //println!("DEBUG:: jump   <{}:{},{}> +{} mode({}, {}, {})", op, a, b, rel_base, mode.0, mode.1, mode.2);
    // jump-if-true | jump-if-false
    // 0n->paramAB, 10n->paramB, 100n->paramA, 110n->immediate
    let mut new_pc: usize = pc + 3;
    if (a != 0 && op_jump_true) || (a == 0 && op_jump_false) {
      new_pc = b as usize;
    }
    return intcodes_internal(outputs, new_pc, inputs, feedback, stdout, &mut rel_base, &mut signals, memory);
  }

  // Get the third operation
  let mut dest: i64 = code[pc + 3];
  if mode.2 == 2 {
    //dest = lookup_memory(dest + *rel_base, &code, &memory);
    dest = dest + *rel_base;
  }
  //println!("DEBUG:: Looked up: dest [{}] = {}", code[pc + 3], dest);
  //println!("DEBUG:: a[{}]={} b[{}]={} dest[{}]={}", code[pc + 1], a, code[pc + 2], b, code[pc + 3], dest);

  if op_less_than || op_equals {
    //println!("DEBUG:: <|=cmp <{}:{} [{}],{} [{}],{} [{}]> +{} mode({}, {}, {})", op, a, code[pc + 1], b, code[pc + 2], dest, code[pc + 3], rel_base, mode.0, mode.1, mode.2);
    let mut result = 0;
    if (op_less_than && a < b) || (op_equals && a == b) {
      result = 1;
    }

    let (outputs, memory) = update_dest(result, dest, outputs, memory);
    return intcodes_internal(outputs, pc + 4, inputs, feedback, stdout, &mut rel_base, &mut signals, memory);
  }
  else if op_add || op_mul {
    //println!("DEBUG:: add|mul<{}:{},{},{}> +{} mode({}, {}, {})", op, a, b, dest, rel_base, mode.0, mode.1, mode.2);
    // add and multiply
    let mut result = a + b;
    if op_mul {
      result = a * b;
    }
    let (outputs, memory) = update_dest(result, dest, outputs, memory);
    return intcodes_internal(outputs, pc + 4, inputs, feedback, stdout, &mut rel_base, &mut signals, memory);
  }
  writeln!(stdout, "Intcode:: Received EXIT (FAILURE)").ok();
  println!("EXIT fallout: op: {} lengths:: code:{} memory:{} signals:{}", op, code.len(), memory.len(), signals.len());
  signals.push(i64::min_value());
  (outputs, signals.to_vec())
}
