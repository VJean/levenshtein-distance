use std::env;
use levenshtein_distance::compute_distance;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    panic!("not enough arguments: please give two strings to compare");
  }
  // The two strings to compare
  let str1 = &args[1];
  let str2 = &args[2];
  // The Levenshtein distance
  let lev = compute_distance(&str1, &str2);

  println!("The Levenshtein distance between [{}] and [{}] is [{}]", str1, str2, lev);
}
