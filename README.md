# Levenshtein Distance

A Rust lib that computes the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two sequences (e.g. the minimum number of character insertion, deletion or substution required to change one sequence into the other).

Example use of the lib:

```
use levenshtein_distance::compute_distance;

fn main() {
  let str1 = String::from("cat");
  let str2 = String::from("bat");
  let lev = compute_distance(&str1, &str2);

  // The Levenshtein distance between [cat] and [bat] is [1]
  println!("The Levenshtein distance between [{}] and [{}] is [{}]", str1, str2, lev);
}
```

The package reads from the command line:

```
$ cargo run -q car chair
The Levenshtein distance between [car] and [chair] is [2]
```
