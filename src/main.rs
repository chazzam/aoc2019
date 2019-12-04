//use std::fs;
use std::convert::TryInto;
//use std::vec::Splice;
//use std::ops::Mul;
//use std::ops::Sub;
//use std::ops::Add;

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
  //let ships_in = std::fs::read_to_string("d1_1.lst").unwrap();
  let ships_in = include_str!("d1_1.lst");

  let sum1 = ships_in
      .lines()
      .fold(0, |acc, x| acc + calc_fuel_mass(x.parse::<i32>().unwrap(), true));
  println!("D1p1:: Final sum of mass: {} (naive)", sum1);

  let sum2 = ships_in
      .lines()
      .fold(0, |acc, x| acc + calc_fuel_mass(x.parse::<i32>().unwrap(), false));
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

fn intcode(inputs: Vec<usize>, pc: usize) -> Vec<usize> {
  // Intcode
  // in set, place 0: opcode
  // opcode 99: halt command processing
  // opcode 1 (add): vec[set[3]] = vec[set[1]] + vec[set[2]]
  // opcode 2 (mul): vec[set[3]] = vec[set[1]] * vec[set[2]]

  let mut outputs = inputs.clone();
  if inputs[pc] < 1 || inputs[pc] > 2 {
    return outputs;
  }

  let dest: usize = inputs[pc + 3]
      .try_into()
      .unwrap();
  let a: &usize = inputs
      .get(inputs[pc + 1])
      .unwrap();
  let b: &usize = inputs
      .get(inputs[pc + 2])
      .unwrap();

  /*if inputs[pc] == 1 {
    outputs[dest] = a + b;
  }
  else if inputs[pc] == 2 {
    outputs[dest] = a * b;
  }*/
  match inputs[pc] {
    1 => outputs[dest] = a + b,
    2 => outputs[dest] = a * b,
    _ => eprintln!("Well, that wasn't supposed to happen"),
  }
  return intcode(outputs, pc + 4);
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

  //let raw_input = std::fs::read_to_string("d2_1.lst").unwrap();
  let raw_input = include_str!("d2_1.lst");

  let mut int_input: Vec<usize> = raw_input
      .split(',')
      .filter_map(|x| x.trim().parse().ok())
      .collect();

  // update inputs:
  int_input[1] = 12;
  int_input[2] = 2;
  //_print_vec(calc_intcode(int_input.clone(), 0), 4);
  println!("D2p1:: {: >12}: 12,  2 -> {: >10}",
      "Inputs", intcode(int_input.clone(), 0)[0]);

  // Find target inputs
  let target = 19690720;
  int_input[2] = 0;
  let mut noun: usize = 0;
  let mut verb: usize = 0;
  for x in 0..=99 {
    int_input[1] = x;
    let result = intcode(int_input.clone(), 0)[0];
    if result <= target && result + 100 >= target {
      noun = x;
      verb = target - result;
      //println!("Found! noun {} and verb {}", x, verb);
      break;
    }
  }
  println!("D2p2:: {: >12}: {: >2}, {: >2} -> {: >10}\n",
      target, noun, verb, 100 * noun + verb);
}

struct Point {
  x: i32,
  y: i32,
}
struct Line (i32, i32, i32, i32);

fn line_intersection(one: &Line, two: &Line) ->
      Option<(i32, i32)> {
  if one.0 == 0 && one.1 == 0 {
    return None;
  }

  let s1: Point = Point {
      x:one.2 - one.0,
      y:one.3 - one.1};
  let s2: Point = Point {
      x:two.2 - two.0,
      y:two.3 - two.1};

  let d:i32 = -s2.x * s1.y + s1.x * s2.y;
  if d == 0 {
    return None;
  }

  let s:i32 = (-s1.y * (one.0 - two.0) + s1.x * (one.1 - two.1)) / d;
  let t:i32 = ( s2.x * (one.1 - two.1) - s2.y * (one.0 - two.0)) / d;

  if s >= 0 && s <= 1 && t >= 0 && t <= 1 {
    // Collision detected
    return Some(
      (one.0 + (t * s1.x),
       one.1 + (t * s1.y))
    );
  }

  None
}

fn day_three() {
  //let raw_input = include_str!("d3_1.lst");
  let raw_input = "R8,U5,L5,D3\nU7,R6,D4,L4\n";
  //let raw_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n";
  let mut raw_lines = raw_input.lines();

  let directionalize = |x: &str| {
    let val: i32 = x[1..]
        .trim()
        .parse()
        .unwrap();
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
    let mut line: Vec<Line> = Vec::new();
    let mut last = Point { x:0, y:0};
    for p in raw.split(',').map(|x| directionalize(x)) {
      line.push(Line (last.x, last.y, last.x + p.x, last.y + p.y));
      last.x += p.x;
      last.y += p.y;
    }
    line
  };

  let line1 = linify(&raw_lines
        .next()
        .unwrap()
        .to_string());
  let line2 = linify(&raw_lines
        .next()
        .unwrap()
        .to_string());
  println!("{}", line1
      .iter()
      .fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));
  println!("{}", line2
      .iter()
      .fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));


  let mut intersects: std::collections::HashSet<(i32,i32)> = std::collections::HashSet::new();
  //let mut intersects: Vec<(i32,i32)> = Vec::new();
  for i in line1[2..].iter() {
    intersects.extend(
      line2.iter()
      .filter_map(|x| line_intersection(i, x)));
  }
  /*let intersects: std::collections::HashSet<(i32, i32)> =
    line1
      .iter()
      .zip(line2.iter())
      .filter_map(|(x, y)| line_intersection(x, y))
      .collect();*/
  println!("intersects: {}", intersects.len());
  print!("[");
  for i in intersects.iter() {
    print!(" ({},{}),", i.0, i.1);
  }
  println!("]");
}

fn main() {
  day_one();
  day_two();
  println!("Hello World!");
  day_three();
}
