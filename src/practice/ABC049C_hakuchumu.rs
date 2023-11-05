use proconio::input;
use regex::Regex;

pub fn exec() {
  input!{
    s: String,
  }

  let mut res = s;

  for t in &["eraser", "erase", "dreamer", "dream"] {
    let re = Regex::new(t).unwrap();
    res = re.replace_all(&res, "").to_string();
  }

  if res.is_empty() {
    println!("YES");
  } else {
    println!("NO");
  }
}