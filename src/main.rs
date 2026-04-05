use std::cmp;
use std::env;

fn compute_distance(str1: &String, str2: &String) -> usize {
  if str1.len() == 0 {
    str2.len()
  } else if str2.len() == 0 {
    str1.len()
  } else {
    // The computation matrix, [row_index][column_index]
    let mut matrix = vec![vec![0; str2.len() + 1]; str1.len() + 1];
    let mut i;
    let mut j;

    // Init matrix with 0..str1.len()+1 as the first row
    // and 0..str2.len()+1 as the first column
    for i in 0..cmp::max(str1.len()+1,str2.len()+1) {
      if i < str1.len()+1 {
        matrix[i][0] = i;
      }
      if i < str2.len()+1 {
        matrix[0][i] = i;
      }
    }

    // first row and first column are initialized,
    // let's fill the rest
    i = 1;
    j = 1;
    for c1 in str1.chars() {
      for c2 in str2.chars() {
        // Substitution cost from c1 to c2 is 1 if they're different, 0 if they're the same
        let cost = {
          if c1 == c2 { 0 }
          else { 1 }
        };

        // current matrix value is the min edition cost between deletion, insertion, or substitution
        matrix[i][j] = cmp::min(matrix[i-1][j] + 1, cmp::min(matrix[i][j-1] + 1,matrix[i-1][j-1] + cost));

        j = j+1; // iterate index to follow c2 position
      }

      i = i+1; // iterate index to follow c1 position
      j = 1;
    }

    // The final Levenshtein distance
    matrix[str1.len()][str2.len()]
  }
}

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
