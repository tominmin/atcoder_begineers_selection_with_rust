use proconio::input;

pub fn exec() {
  input!{
    n: usize,
    a: [usize; n],
  };

  let mut b = 0;
  let mut ans = 0;

  for i in 0..n {
    b = a[i] | b;
  }

  /*
    2 => 0010, 6 => 0110
    4 => 0100,
    8 => 1000,
    If you repeatedly divide any number by 2,
    you can divide as many trailing 0s as possible using binary system.
  */

  while (b & 1) == 0 {
    b = b >> 1;
    ans += 1;
  }

  println!("{}", ans);
} 