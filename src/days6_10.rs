use std::collections::{HashMap, HashSet};

pub fn run_days() {
  day_six();
}


fn binomial(x: usize) -> usize {
  (1..(x + 1)).fold(0, |acc, x| acc + x)
}

fn count_orbits(input: &str) ->(HashMap<&str, String>, HashMap<&str, String>, usize) {
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
  (direct_orbits, orbits, dashes)
}

pub fn day_six() {
  let test_input = include_str!("inputs/d6_test.lst").trim(); // 42

  let (tdirects, torbits, tdashes) = count_orbits(test_input);
  println!("directs: {} indirects: {} test: {} dashes: {}", tdirects.len(), torbits.len(),
  torbits.len() + tdirects.len(), tdashes);
  //test_input
  
  // 869218 and 868157 too high
  // 2473 too low
  let input = include_str!("inputs/d6_1.lst").trim();
  let (directs, orbits, dashes) = count_orbits(input);
  println!("directs: {} indirects: {} sum: {} dashes: {}", directs.len(), orbits.len(),
  orbits.len() + directs.len(), dashes);
 
  let input2 = include_str!("inputs/d6_2_test.lst").trim();
  let (_, orbits2, _) = count_orbits(input2);

 
  let you2: HashSet<&str> = orbits2.get("YOU").unwrap().split("-").collect();
  let san2: HashSet<&str> = orbits2.get("SAN").unwrap().split("-").collect();
  for i in you2.difference(&san2) {
    print!(" {},", i);
  }
  println!("");
  for i in you2.symmetric_difference(&san2) {
    print!(" {},", i);
  }
  println!("\ndiff: {} you: {}; san: {};", you2.difference(&san2).count(),
    orbits2.get("YOU").unwrap(),
    orbits2.get("SAN").unwrap());


  // 208 too low
  let you: HashSet<&str> = orbits.get("YOU").unwrap().split("-").collect();
  let san: HashSet<&str> = orbits.get("SAN").unwrap().split("-").collect();
  /*for i in you.difference(&san) {
    print!(" {},", i);
  }
  println!("");
  for i in you.symmetric_difference(&san) {
    print!(" {},", i);
  }*/
  println!("\ndiff: {} sym_diff: {}\nyou: {};\nsan: {};",
    you.difference(&san).count(),
    you.symmetric_difference(&san).count() - 2,
    orbits.get("YOU").unwrap(),
    orbits.get("SAN").unwrap());
}