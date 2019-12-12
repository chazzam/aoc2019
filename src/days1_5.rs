use intcode;
use std::io;
use generic;

pub struct Point {
  x: i32,
  y: i32,
}
pub struct Line (i32, i32, i32, i32);

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

pub fn distance(ax:i32, ay:i32, bx:i32, by:i32) -> i32 {
  f64::from((bx - ax).pow(2) + (by - ay).pow(2)).sqrt().floor() as i32
}

fn line_intersection(one: &Line, two: &Line, steps1: i32, steps2: i32) ->
    Option<(i32, i32, i32)> {
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
  let x: i32 = p0.x + (t * f64::from(s1.x)) as i32;
  let y: i32 = p0.y + (t * f64::from(s1.y)) as i32;
  let walk_back1: i32 = distance(x, y, p1.x, p1.y);
  let walk_back2: i32 = distance(x, y, p3.x, p3.y);
  let steps = (steps1 + steps2 - walk_back1.abs() - walk_back2.abs()).abs();
  //println!("s1:{} s2:{} w1:{} w2:{} s:{}", steps1, steps2, walk_back1, walk_back2, steps);
  return Some((x, y, steps));
}

fn manhattanize(input: &str) -> Vec<(i32, i32)> {
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

    let mut distances: Vec<(i32, i32)> = Vec::new();
    // Add tracking distance down each line
    // work in how to get distance down a segment to intersection
    let mut steps1: i32 = 0;
    let mut steps2: i32;
    for i in line1.iter() {
      steps1 += distance(i.0, i.1, i.2, i.3);
      steps2 = 0;
      //let mut s2 = &steps2;
      distances.extend(
        line2.iter()
        .filter_map(|x| {
          steps2 = steps2 + distance(x.0, x.1, x.2, x.3);
          line_intersection(i, x, steps1, steps2) })
        .map(|(x,y,s)| (x.abs() + y.abs(), s)));
    }
    // update sort to work on a tuple...
    distances.sort_by(|(m1, _), (m2, _)| m1.partial_cmp(m2).unwrap());
    return distances;
}

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
  println!("D2p1:: {: >12}: 12,  2 -> {: >10}",
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
  println!("D2p2:: {: >12}: {: >2}, {: >2} -> {: >10}\n",
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
  //_print_vec(intcode([3,0,4,0,99].to_vec(), 0), 4);
  //_print_vec(intcode([1002,4,3,4,33].to_vec(), 0), 4);
  let input = intcode::int_input(include_str!("inputs/d5_1.lst"));
  /*let int_input: Vec<i64> = raw_input
    .split(',')
    .filter_map(|x| x.trim().parse().ok())
    .collect();*/
  println!("D5p1:: Intcode() Input '1'");
  println!("D5p1:: Input=1, Result: {: >10}\n",
    intcode::intcodes(input.clone(), 0, [1].to_vec()).1.last().unwrap());
  //println!("D5p2:: Intcode() Input '5'");
  println!("D5p2:: Input=5, Result: {: >10}\n\n",
    intcode::intcodesq(input, 0, [5].to_vec()).1.last().unwrap());
}

pub fn run_days() {
  day_one();
  day_two();
  day_three();
  day_four();
  day_five();
}