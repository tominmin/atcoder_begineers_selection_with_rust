use proconio::input;

pub fn exec() {
  input!{
    n: usize,
    a: usize,
    b: usize
  }

  let mut ans = 0;

  for i in 1..=n {
    let mut dig = i;
    let mut sum = 0;
    
    while dig > 0 {
      sum += dig % 10;
      dig /= 10;
      if dig == 0 {
        break;
      }
    }

    if a <= sum && sum <= b {
      ans += i;
    }
  }

  println!("{:?}", ans);
}