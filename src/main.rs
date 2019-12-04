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
  if one.0 == 0 && one.1 == 0 && two.0 == 0 && two.1 == 0 {
    return None;
  }

  let p0 = Point { x:one.0, y:one.1};
  let p1 = Point { x:one.2, y:one.3};
  let p2 = Point { x:two.0, y:two.1};
  let p3 = Point { x:two.2, y:two.3};

  let s1 = Point {
      x:p1.x - p0.x,
      y:p1.y - p0.y};
  let s2 = Point {
      x:p3.x - p2.x,
      y:p3.y - p2.y};

  //float s, t;
  let d:i32 = s1.x * s2.y - s2.x * s1.y;
  if d == 0 {
    return None;
  }
  let d_pos = d > 0;

  let s02 = Point {
    x: p0.x - p2.x,
    y: p0.y - p2.y,
  };
  let s_numerator = s1.x * s02.y - s1.y * s02.x;
  if (s_numerator < 0) == d_pos {
    return None;
  }

  let t_numerator = s2.x * s02.y - s2.y * s02.x;
  if (t_numerator < 0) == d_pos {
    return None;
  }

  if (s_numerator > d) == d_pos || (t_numerator > d) == d_pos {
    return None;
  }

  let t:f64 = f64::from(t_numerator) / f64::from(d);

  // Collision detected
  return Some(
    (p0.x + (t * f64::from(s1.x)) as i32,
     p0.y + (t * f64::from(s1.y)) as i32)
  );
}

fn day_three() {
  let manhattanize = |input: &str| {
    let mut raw_lines = input.lines();

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
    /* print the line lists
    println!("{}", line1
        .iter()
        .fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));
    println!("{}", line2
        .iter()
        .fold(String::new(), |acc, x| acc + &format!(" ({}, {}, {}, {});", x.0, x.1, x.2, x.3).to_string()));
    */

    /* Get the full intersections + distance list
    let mut intersects: std::collections::HashSet<(i32,i32)> = std::collections::HashSet::new();
    for i in line1.iter() {
      intersects.extend(
        line2.iter()
        .filter_map(|x| line_intersection(i, x)));
    }

    println!("intersects: [");
    for i in intersects.iter() {
      print!(" ({},{})->{},", i.0, i.1, i.0.abs()+i.1.abs());
    }
    println!("\n]");
    */

    let mut distances: Vec<i32> = Vec::new();
    for i in line1.iter() {
      distances.extend(
        line2.iter()
        .filter_map(|x| line_intersection(i, x))
        .map(|(x,y)| x.abs() + y.abs()));
    }
    distances.sort();
    return distances;
  };

  println!("D3p1::Test 1:   6 ?= {: >4}", manhattanize("R8,U5,L5,D3\nU7,R6,D4,L4\n")[0]);
  println!("D3p1::Test 2: 159 ?= {: >4}", manhattanize("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n")[0]);
  println!("D3p1::Test 3: 135 ?= {: >4}", manhattanize("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n")[0]);

  let raw_input = include_str!("d3_1.lst"); // 554 too high
  let intersects = manhattanize(raw_input);

  println!("D3p1:: {} intersections; distance: {}", intersects.len(), intersects[0]);
}

fn main() {
  day_one();
  day_two();
  println!("Hello World!");
  day_three();
}
