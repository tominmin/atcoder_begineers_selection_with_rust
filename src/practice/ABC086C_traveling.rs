use proconio::input;

pub fn exec() {
  input!{
    n: usize,
    v: [[i32;3];n],
  }

  let mut bt = 0;
  let mut bx = 0;
  let mut by = 0;
  let mut b = false;

  for p in 0..n {
    let t = v[p][0];
    let x = v[p][1];
    let y = v[p][2];

    let mp = t - bt;
    let cp = (x - bx).abs() + (y - by).abs();

    if mp < cp {
      b = true;
      break;
    }

    if mp % 2 != cp % 2 {
      b = true;
      break;
    }

    bt = t;
    bx = x;
    by = y;
  }

  if b != true {
    println!("Yes");
  } else {
    println!("No");
  }
}