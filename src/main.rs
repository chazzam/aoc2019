//use std::fs;
use std::convert::TryInto;
//use std::vec::Splice;

// Day one, naive does one calclation
// not naive continues calculating using the result
fn calc_fuel_mass(mass: i32, naive: bool) -> i32 {
  let fuel_mass = (mass / 3) - 2;

  if naive {
    return fuel_mass;
  } else if fuel_mass <= 0 {
    return 0;
  }
  return fuel_mass + calc_fuel_mass(fuel_mass, true);
}

fn day_one() {
  let ships_in = std::fs::read_to_string("d1_1.lst").unwrap();

  let sum1 = ships_in.lines().fold(0, |acc, x| acc + calc_fuel_mass(x.parse::<i32>().unwrap(), true));
  println!("D1p1:: Final sum of mass: {} (naive)", sum1);

  let sum2 = ships_in.lines().fold(0, |acc, x| acc + calc_fuel_mass(x.parse::<i32>().unwrap(), false));
  println!("D1p2:: Final sum of mass: {}\n", sum2);

  /*let raw_mass = 1969;
  println!("fuel mass: {} to {}", raw_mass, calc_fuel_mass(raw_mass, true));*/
}

fn _print_vec(disp: Vec<usize>, width: usize) {
  println!("[");
  for i in disp.iter().enumerate() {
    print!(" {: >4},", i.1);
    if (i.0 != 0) && (((i.0 + 1) % width) == 0) {
      println!("");
    }
  }
  println!("\n]");
  return;
}

fn intcode(inputs: Vec<usize>, ptr: usize) -> Vec<usize> {
  // Intcode
  // in set, place 0: opcode
  // opcode 99: halt command processing
  // opcode 1 (add): vec[set[3]] = vec[set[1]] + vec[set[2]]
  // opcode 2 (mul): vec[set[3]] = vec[set[1]] * vec[set[2]]

  let mut outputs = inputs.clone();
  if inputs[ptr] < 1 || inputs[ptr] > 2 {
    return outputs;
  }

  let dest: usize = inputs[ptr + 3].try_into().unwrap();
  let a: &usize = inputs.get(inputs[ptr + 1]).unwrap();
  let b: &usize = inputs.get(inputs[ptr + 2]).unwrap();

  /*if inputs[ptr] == 1 {
    outputs[dest] = a + b;
  }
  else if inputs[ptr] == 2 {
    outputs[dest] = a * b;
  }*/
  match inputs[ptr] {
    1 => outputs[dest] = a + b,
    2 => outputs[dest] = a * b,
    _ => eprintln!("Well, that wasn't supposed to happen"),
  }
  return intcode(outputs, ptr + 4);
}

fn day_two() {
  /*
  // Test 1
  _print_vec(intcode([1,0,0,0,99].to_vec(), 0), 4);
  // Test 2
  _print_vec(intcode([2,3,0,3,99].to_vec(), 0), 4);
  // Test 3
  _print_vec(intcode([2,4,4,5,99,0].to_vec(), 0), 4);
  // Test 4
  _print_vec(intcode([1,1,1,4,99,5,6,0,99].to_vec(), 0), 4);
  */

  let raw_input = std::fs::read_to_string("d2_1.lst").unwrap();

  let mut int_input: Vec<usize> = raw_input.split(',').map(|x| x.trim().parse().unwrap()).collect();

  // update inputs:
  int_input[1] = 12;
  int_input[2] = 2;
  //_print_vec(calc_intcode(int_input.clone(), 0), 4);
  println!("D2p1:: Inputs: 12,  2 -> {: >10}", intcode(int_input.clone(), 0)[0]);

  int_input[1] = 70;
  int_input[2] = 14;
  println!("D2p2:: Inputs: 70, 14 -> {: >10};\n       100 * 70 + 14 = {}\n", intcode(int_input.clone(), 0)[0], 100 * 70 + 14);
}

struct Point {
  x: i32,
  y: i32,
}
struct Line<T> (T,T,T,T);

fn day_three() {
  // convert to coordinates?
  // L/R - (-X/+X);
  // U/D - (+Y/-Y);
  // R75 -> (+75,0)
  //let raw_input = std::fs::read_to_string("d3_1.lst").unwrap();
  let raw_input = "R8,U5,L5,D3\nU7,R6,D4,L4\n";
  //let raw_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n";
  let mut raw_lines = raw_input.lines();

  let directionalize = |x: &str| {
    let val: i32 = x[1..].trim().parse().unwrap();
    let mut z = Point { x:0, y:0};
    match x.chars().nth(0).unwrap() {
      'R' => z.x =  val,
      'L' => z.x = -val,
      'U' => z.y =  val,
       _  => z.y = -val,
    }
    z
  };

  let linify = |raw: &String| {
    let mut line: Vec<Line<i32>> = Vec::new();
    let mut last = Point { x:0, y:0};
    for p in raw.split(',').map(|x| directionalize(x)) {
      line.push(Line (last.x, last.y, last.x + p.x, last.y + p.y));
      last.x += p.x;
      last.y += p.y;
    }
    line
  };

  let line1 = linify(&raw_lines.next().unwrap().to_string());
  let line2 = linify(&raw_lines.next().unwrap().to_string());
  println!("{}", line1.iter().fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));
  println!("{}", line2.iter().fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));

}

fn main() {
  day_one();
  day_two();
  println!("Hello World!");
  day_three();
}