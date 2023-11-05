use proconio::input;

pub fn exec() {
  input!{
    n: usize,
    mut a: [usize; n],
  };

  a.sort();
  a.reverse();

  let mut alice_point = 0;
  let mut bob_point = 0;

  for i in 0..n {
    if i % 2 == 0 {
      alice_point += a[i];
    } else {
      bob_point += a[i];
    }
  }

  let ans = alice_point - bob_point;
  println!("{}", ans);
}