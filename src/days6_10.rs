use intcode;
use std::collections::{HashMap, HashSet};
//use std::thread;
//use std::sync::mpsc;
use std::ops::Add;
//use generic;

pub fn run_days() {
  _day_six();
  println!("Hello Testing!");
  _day_seven();
  _day_eight();
  _day_nine();
  _day_ten();
}


fn _binomial(x: usize) -> usize {
  (1..(x + 1)).fold(0, |acc, x| acc + x)
}

fn _count_orbits(input: &str) ->(HashMap<&str, String>, usize) {
  let mut direct_orbits: HashMap<&str, String> = HashMap::new();
  for i in input
      .lines()
      .map(|x:&str| x.split(")")
      .collect::<Vec<&str>>()) {
    let k = i[0];
    let v = i[1];
    let nv = format!("{}-{}", v, k).to_string();
    //println!("direct_orbits adding {}={}", v, nv);
    direct_orbits.insert(v, nv);
  }
  let mut dashes = 0;
  let mut orbits: HashMap<&str, String> = HashMap::new();
  for (k, v) in direct_orbits.iter() {
    // looks like k="A", v="A-B", want to get to "B"
    let mut lv = v;
    let mut nv: String = k.to_string();
    while !lv.contains(&"COM".to_string()) {
      let ds: Vec<&str> = lv.split("-").collect();
      let nk = ds[ds.len() - 1];
      lv = direct_orbits.get(nk).unwrap();
      nv += &format!("-{}", nk).to_string();
      //println!("k:{} lv:{} nv: {}", k,lv, nv);
    }
    nv += &"-COM".to_string();
    dashes += nv.matches("-").count();
    orbits.insert(k, nv.to_string());
  }
  (orbits, dashes)
}

pub fn _day_six() {
  // test p1
  let mut input = include_str!("inputs/d6_test.lst").trim(); // 42
  let (orbits, dashes) = _count_orbits(input);
  println!("TEST::D6p1:: directs: {} indirects: {} all: {} ?= 42", 
    orbits.len(),
    dashes - orbits.len(),
    dashes);
  // test p2
  input = include_str!("inputs/d6_2_test.lst").trim();
  let (orbits, _) = _count_orbits(input);
  let mut you: HashSet<&str> = orbits
      .get("YOU").unwrap().split("-").collect();
  let mut san: HashSet<&str> = orbits
      .get("SAN").unwrap().split("-").collect();
  /*for i in you.difference(&san) {
    print!(" {},", i);
  }
  println!("");
  for i in you2.symmetric_difference(&san2) {
    print!(" {},", i);
  }*/
  println!("TEST::D6p2:: orbit changes: {} ?= 4 you: {}; san: {};\n",
    you.symmetric_difference(&san).count() - 2,
    orbits.get("YOU").unwrap(),
    orbits.get("SAN").unwrap());
  //test_input
  
  // 869218 and 868157 too high
  // 2473 too low
  input = include_str!("inputs/d6_1.lst").trim();
  let (orbits, dashes) = _count_orbits(input);
  println!("D6p1:: Orbits:: Direct: {} Indirect: {} All: {}",
    orbits.len(),
    dashes - orbits.len(),
    dashes);
 
  // 208 too low
  you = orbits.get("YOU").unwrap().split("-").collect();
  san = orbits.get("SAN").unwrap().split("-").collect();
  /*for i in you.difference(&san) {
    print!(" {},", i);
  }
  println!("");
  for i in you.symmetric_difference(&san) {
    print!(" {},", i);
  }*/
  println!("D6p2:: Orbit Changes: {}\n",
    you.symmetric_difference(&san).count() - 2);
}

pub fn _day_seven() {
  let calc_thrusters = |phase: Vec<i64>, code: Vec<i64>| -> i64 {
    let mut last_input: i64 = 0;
    for i in phase.iter() {
      let rintcode = intcode::intcodesq(
        code.clone(), 
        0, 
        [*i as i64, last_input].to_vec());
      last_input = rintcode.1[0];
    }
    last_input
  };

  let mut input = intcode::int_input(
    include_str!("inputs/d7_1-1.test"));
  println!("TEST::D7p1-1:: Result: {: >10} ?= 43210",
    calc_thrusters([4,3,2,1,0].to_vec(), input.clone()));

  input = intcode::int_input(
    include_str!("inputs/d7_1-2.test"));
  println!("TEST::D7p1-2:: Result: {: >10} ?= 54321",
    calc_thrusters([0,1,2,3,4].to_vec(), input.clone()));

  input = intcode::int_input(
    include_str!("inputs/d7_1-3.test"));
  println!("TEST::D7p1-2:: Result: {: >10} ?= 65210",
    calc_thrusters([1,0,4,3,2].to_vec(), input.clone()));
  let calc_thrusters_feedback = |phase: Vec<i64>, code: Vec<i64>| -> i64 {
    let mut last_input: i64 = 0;
    let mut signals = Vec::<i64>::new();
    for i in phase.iter() {
      let rintcode = intcode::intcodesf(
        code.clone(), 
        0, 
        [*i as i64, last_input].to_vec(),
        &mut signals);
      last_input = rintcode.1[0];
      // make sure to clear the signals between each run
      signals.clear();
    }
    last_input
  };

  input = intcode::int_input("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
  println!("TEST::D7p2-1:: Result: '139629729' ?= {: >10}\n",
    calc_thrusters_feedback([9,8,7,6,5].to_vec(), input.clone()));


  let permutate_phase = 
    |   in_phase: Vec::<i64>, code: Vec::<i64>, 
        f: &dyn Fn(Vec<i64>, Vec<i64>) 
      -> i64| {
    let mut phase = in_phase;
    let mut results = Vec::<i64>::new();
    for run in 0..120 {
      results.push(f(phase.clone(), code.clone()));
      /*println!("D7p1_{}:: phase: {} Result: {: >10}", run,
        format!("[{},{},{},{},{}]", phase[0], phase[1], phase[2], phase[3], phase[4]),
        results.last().unwrap());*/
      let z = run % 6;
      match z {
        0 => phase.swap(0, run % 5),
        1 => phase.swap(1, 2),
        2 => phase.swap(1, 3),
        3 => phase.swap(1, 4),
        4 => phase.swap(2, 3),
        5 => phase.swap(3, 4),
        _ => (),
      }
    }
    results.sort();
    return results;
  };
  // 44804, 64401 too low
  input = intcode::int_input(
    include_str!("inputs/d7_1.lst"));
  let results = permutate_phase([0,1,2,3,4].to_vec(), input.clone(), &calc_thrusters);
  println!("D7p1:: Highest Thrust #: {} out of: {}", results.last().unwrap(), results.len());

  let results = permutate_phase([5, 6, 7, 8, 9].to_vec(), input, &calc_thrusters_feedback);
  println!("D7p_:: Highest Thrust #: {} out of: {}\n", results.last().unwrap(), results.len());
}

/*#[derive(Clone)]
struct Img { i:Vec::<Color>, h: usize, w: usize }
impl std::fmt::Display for Img {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = format!("[\n");
    for i in self.i.iter().enumerate() {
      s = format!("{}{}", s, i.1);
      if i.0 != 0 && ((i.0 + 1) % self.w) == 0 && i.0 + 1 != self.i.len() {
        s = s + "\n";
      }
      if self.h > 0 && (i.0 + 1) % (self.h * self.w) == 0 && i.0 + 1 != self.i.len() {
        s = s + "\n";
      }
    }
    s = s + "\n]";
    write!(f, "{}", s)
  }
}
pub trait Summary {
  fn summarize(&self) -> Self;
}
impl Summary for Img {
  fn summarize(&self) -> Self {
    let mut iter = self.i.rchunks(self.h * self.w);
    let mut final_layer = Img { h: self.h, w: self.w, 
      i:iter.next().unwrap().to_vec() };
    for layer in iter {
      for (i, x) in layer.iter().enumerate() {
        final_layer.i[i] += *x;
      }
    }
    final_layer
  }
}*/

#[derive(Clone)]
struct Img { i:Vec::<i64>, h: usize, w: usize }
impl std::fmt::Display for Img {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = format!("[\n");
    for (i, x) in self.i.iter().enumerate() {
      let c: &str = if *x == 1 { "\x1b[97m" } else { "\x1b[30m" };
      s = format!("{}{}{}", s, c, *x);
      if i != 0 && ((i + 1) % self.w) == 0 && i + 1 != self.i.len() {
        s = s + "\n";
      }
      if self.h > 0 && (i + 1) % (self.h * self.w) == 0 && i + 1 != self.i.len() {
        s = s + "\n";
      }
    }
    s = s + "\x1b[0m\n]";
    write!(f, "{}", s)
  }
}
pub trait Summary {
  fn summarize(&self) -> Self;
}
impl Summary for Img {
  fn summarize(&self) -> Self {
    let mut iter = self.i.rchunks(self.h * self.w);
    let mut final_layer = Img { h: self.h, w: self.w, 
      i:iter.next().unwrap().to_vec() };
    for layer in iter {
      /*for (i, x) in layer.iter().enumerate() {
        final_layer.i[i] += x;
      }*/
      let img = Img { w:final_layer.w, h: final_layer.h, i: layer.to_vec() };
      final_layer += img;
    }
    final_layer
  }
}
impl std::ops::AddAssign for Img {
  // black - 0; white - 1; transparent - 2
  fn add_assign(&mut self, other: Self) {
    if self.h != other.h && self.w != other.w {
      return;
    }
    for (i, x) in self.i.clone().iter().enumerate() {
      let o = &other.i[i];
      self.i[i] = if *x == 0 && *o == 1 { *o } else if x <= o { *x } else { *o } 
    }
  }
}

/*#[derive(Clone, Copy)]
struct Color { x: i64 }
impl std::ops::AddAssign for Color {
  // black - 0; white - 1; transparent - 2
  fn add_assign(&mut self, other: Self) {
    *self = Self {
        x: if self.x == 0 && other.x == 1 { other.x } else if self.x <= other.x { self.x } else { other.x }
    }
  }
}
impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let c: &str = if self.x == 1 { "\x1b[97m" } else { "\x1b[30m" };
    write!(f, "{}{}\x1b[0m", c, self.x)
  }
}
impl std::str::FromStr for Color {
  type Err = std::num::ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let x_from = s.trim().parse::<i64>()?;
    Ok(Color { x: x_from })
  }
}*/

pub fn _day_eight() {
  // 3 x 2
  let input = Img {h:2, w:3,
    i:"123456789012"
      .split("")
      .filter_map(|x| x.trim().parse().ok())
      .collect()};
  println!("TEST:D8p1-1:\n{}", input);

  let input = Img { h: 2, w: 2,
    i: "0222112222120000".trim()
      .split("")
      .filter_map(|x| x.trim().parse().ok())
      .collect() };
  println!("TEST::D8p2-1:\n{}", input.summarize());

  // 25 x 6 pixels
  let layers = 25 * 6;
  let input = Img {w:25, h:6,
      i:include_str!("inputs/d8_1.lst")
      .split("")
      .filter_map(|x| x.trim().parse().ok())
      .collect()};

  // need to count 0s, 1s, and 2s
  let mut counts: Vec::<HashMap<i64, i64>> = Vec::new();
  for (i, x) in input.i.iter().enumerate() {
    let layer: usize = i / layers;
    if counts.len() == 0 || counts.len() - 1 < layer {
      counts.push(HashMap::new());
    }
    /*if counts[layer].get(&x.x).is_some() {
      let update = counts[layer].get(&x.x).unwrap() + 1;
      counts[layer].insert(x.x, update);
    } else {
      counts[layer].insert(x.x, 1);
    }*/
    if counts[layer].get(x).is_some() {
      let update = counts[layer].get(x).unwrap() + 1;
      counts[layer].insert(*x, update);
    } else {
      counts[layer].insert(*x, 1);
    }
  }
  let mut min_zeros: usize = 0;
  let mut min: i64 = layers as i64;
  for (k, v) in counts.iter().enumerate() {
    if v.get(&0).is_some() {
      let x = v.get(&0).unwrap();
      if *x < min {
        min = *x;
        min_zeros = k;
      }
    }
  }
  let ones = counts[min_zeros].get(&1).unwrap();
  let twos = counts[min_zeros].get(&2).unwrap();
  println!("D8p1:: MinZeroLayer: {} Zeros: {} Ones {} Twos {} Val: {}",
    min_zeros, min, ones, twos, ones * twos);
  println!("D8p2::\n{}", input.summarize());
  //generic::_print_vect(input.summarize().i, input.w, input.h);
}

pub fn _day_nine() {
  ()
}

pub fn _day_ten() {
  ()
}
