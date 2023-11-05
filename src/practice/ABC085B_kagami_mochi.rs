use proconio::input;

pub fn exec() {
  input!{
    n: usize,
    mut a: [i32;n]
  }

  a.sort();
  a.reverse();

  let mut x = 1;
  let mut now = 0;
  for v in a {
    x += if now > v { 1 } else { 0 };
    now = v;
  }
  println! {"{}",x}
}