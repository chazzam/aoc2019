use intcode;
use std::io;
use generic::{calc_fuel_mass, manhattanize, _print_vec};

pub fn day_one() {
  //let ships_in = std::fs::read_to_string("d1_1.lst").unwrap();
  let ships_in = include_str!("inputs/d1_1.lst");

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

pub fn day_two() {
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
  let raw_input = include_str!("inputs/d2_1.lst");

  let mut int_input: Vec<i64> = raw_input
      .split(',')
      .filter_map(|x| x.trim().parse().ok())
      .collect();

  println!("D2p1:: Intcode() Noun=12, Verb=2");
  // update noun & verb inputs:
  int_input[1] = 12;
  int_input[2] = 2;
  //_print_vec(calc_intcode(int_input.clone(), 0), 4);
  println!("D2p1:: {: >12}: 12,  2 -> {: >10} ?==3654868",
      "Inputs", intcode::intcode(int_input.clone(), 0).0[0]);

  // Find target inputs
  let target = 19690720;
  int_input[2] = 0;
  let mut noun: i64 = 0;
  let mut verb: i64 = 0;

  let empty = Vec::<i64>::new();
  for x in 0..=99 {
    int_input[1] = x;
    let result = intcode::intcodesw(int_input.clone(), 0, empty.clone(), &mut io::sink()).0[0];
    if result <= target && result + 100 >= target {
      noun = x;
      verb = target - result;
      //println!("Found! noun {} and verb {}", x, verb);
      break;
    }
  }
  println!("D2p2:: {: >12}: {: >2}, {: >2} -> {: >10} ?==7014\n",
      target, noun, verb, 100 * noun + verb);
}

pub fn day_three() {
  /* tests part 1 
  let mut t1 = manhattanize("R8,U5,L5,D3\nU7,R6,D4,L4\n");
  println!("D3p1::Test 1:   6 ?= {: >4}; {}", t1[0].0, t1[0].1);

  let mut t2 = manhattanize("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n");
  println!("D3p1::Test 2: 159 ?= {: >4}", t2[0].0);

  let mut t3 = manhattanize("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n");
  println!("D3p1::Test 3: 135 ?= {: >4}", t3[0].0);
  // tests part 2
  println!("");
  t1.sort_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
  println!("D3p2::Test 1: {} intersections; distance: {}; steps: 30 ?= {}", t1.len(), t1[0].0, t1[0].1);
  t2.sort_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
  println!("D3p2::Test 2: {} intersections; distance: {}; steps: 610 ?= {}", t2.len(), t2[0].0, t2[0].1);
  t3.sort_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
  println!("D3p2::Test 2: {} intersections; distance: {}; steps: 410 ?= {}", t3.len(), t3[0].0, t3[0].1);
  */

  let raw_input = include_str!("inputs/d3_1.lst"); // 554 too high
  let mut intersects = manhattanize(raw_input);

  println!("D3p1:: {: >4} intersections; distance: {: >6}; steps: {: >6}", intersects.len(), intersects[0].0, intersects[0].1);

  // 31908 too low
  intersects.sort_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
  println!("D3p2:: {: >4} intersections; distance: {: >6}; steps: {: >6}\n", intersects.len(), intersects[0].0, intersects[0].1);
}

pub fn day_four() {
  let test_password = |x:i32, strict:bool| -> Option<i32> {
    // six digits
    //if x.to_string().len() != 6 {
    //  return None;
    //}
    /*let xstr = x.to_string();
    let xs: Vec<i32> = xstr
      .trim()
      .split("")
      .filter_map(|x| if x.len() > 0 { Some(x.trim().parse().unwrap()) } None )
      .collect();*/
    let xstr: &str = &x.to_string();
    let xs: Vec<_> = xstr
      .chars()
      .map(|d| d.to_digit(10).unwrap())
      .collect();
    if xs.len() != 6 {
      return None;
    }
    let mut xi = xs.iter();
    let mut last:u32 = *xi.next().unwrap();
    let mut adjacent = false;
    let mut strict_adjacent = false;
    for i in xi {
      if (*i as i64) - (last as i64) < 0 {
        return None;
      }
      if *i == last {
        adjacent = true;
      }
      //println!("x:{} i:{} matches:{}", x, i, xstr.matches(i.to_string().repeat(2).as_str()).count());
      if strict {
        let doubles = xstr.matches(i.to_string().repeat(2).as_str()).count();
        let triples = xstr.matches(i.to_string().repeat(3).as_str()).count();
        if doubles == 1 && triples == 0 {
          strict_adjacent = true;
        }
      }
      last = *i;
    }
    if !adjacent {
      return None;
    }
    if strict && !strict_adjacent {
      return None;
    }
    // adjacent repeated digit
    // never decreases from left to right
    Some(x)
  };
  /* run Day 4 part 1 tests
  // for p1, only 111111 is valid. for p2, none are valid
  let mut tests: Vec<i32> = [111111, 223450, 123789].to_vec();
  // for p1, all three valid. for p2, only 112233 and 111122 are valid
  tests.extend([112233, 123444, 111122].to_vec());
  let mut test_results: Vec<_> = tests
    .iter()
    .filter_map(|x| test_password(*x, false))
    .collect();
  for x in test_results.iter() {
    println!("D4p1 valid {}", x);
  }
  test_results = tests
    .iter()
    .filter_map(|x| test_password(*x, true))
    .collect();
  for x in test_results.iter() {
    println!("D4p2 valid {}", x);
  }
  */

  /*let input: Vec<_> = (382345..843167).collect();
  let results: Vec<_> = input
    .iter()
    .filter_map(|x| test_password(*x, false))
    .collect();*/
  let results: Vec<_> = 
    (382345..843167)
    .filter_map(|x| test_password(x, false))
    .collect();
  println!("D4p1:: passwords: {: >5} (any adjacent)", results.len());
  // we can re-use the previous results here, as we're only getting more strict
  let p2_results: Vec<_> = results
    .iter()
    .filter_map(|x| test_password(*x, true))
    .collect();
  println!("D4p2:: passwords: {: >5} (strictly adjacent)\n", p2_results.len());
  // 364 too high
}

pub fn day_five() {
  _print_vec(intcode::intcodesq([3,0,4,0,99].to_vec(), 0, [99].to_vec()).1, 4);
  _print_vec(intcode::intcode([1002,4,3,4,33].to_vec(), 0).1, 4);
  let input = intcode::int_input(include_str!("inputs/d5_1.lst"));
  /*let int_input: Vec<i64> = raw_input
    .split(',')
    .filter_map(|x| x.trim().parse().ok())
    .collect();*/
  println!("D5p1:: Intcode() Input '1'");
  println!("D5p1:: Input=1, Result: {: >10} ?==15386262\n",
    intcode::intcodes(input.clone(), 0, [1].to_vec()).1.last().unwrap());
  //println!("D5p2:: Intcode() Input '5'");
  println!("D5p2:: Input=5, Result: {: >10} ?==10376124\n\n",
    intcode::intcodesq(input, 0, [5].to_vec()).1.last().unwrap());
}

pub fn run_days() {
  day_one();
  day_two();
  day_three();
  day_four();
  day_five();
}