use proconio::input;

pub fn exec() {
  input!{
    a: u32,
    b: u32,
    c: u32,
    d: u32,
  }

  let mut ans = 0;
  let mut sum = 0;

  for i in 0..a + 1 {
    for j in 0..b + 1 {
      for k in 0..c + 1 {
        sum = (i * 500) + (j * 100) + (k * 50);
        if sum == d {
          ans += 1;
        }
      }
    }
  }

  println!("{}", ans);
}