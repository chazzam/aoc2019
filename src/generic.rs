pub struct Point {
  pub x: i32,
  pub y: i32,
}
pub struct Line (pub i32, pub i32, pub i32, pub i32);

// Day one, naive does one calclation
// not naive continues calculating using the result
pub fn calc_fuel_mass(mass: i32, naive: bool) -> i32 {
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

pub fn line_intersection(one: &Line, two: &Line, steps1: i32, steps2: i32) ->
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

pub fn manhattanize(input: &str) -> Vec<(i32, i32)> {
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

pub fn _print_vec(disp: Vec<i64>, width: usize) {
  _print_vech(disp, width, 0);
}

pub fn _print_vech(disp: Vec<i64>, width: usize, height: usize) {
  println!("[");
  for i in disp.iter().enumerate() {
    print!(" {: >4},", i.1);
    if i.0 != 0 && ((i.0 + 1) % width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
    if height > 0 && (i.0 + 1) % (height * width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
  }
  println!("\n]");
  return;
}

pub fn _print_vect<T>(disp: Vec<T>, width: usize, height: usize)
    where T: std::fmt::Display {
  println!("[");
  for i in disp.iter().enumerate() {
    print!("{: >4},", i.1);
    if i.0 != 0 && ((i.0 + 1) % width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
    if height > 0 && (i.0 + 1) % (height * width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
  }
  println!("\n]");
  return;
}
