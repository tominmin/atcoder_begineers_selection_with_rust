use proconio::input;

pub fn exec() {
  input!{
    abc: i8,
  };

  let a = abc / 100;
  let b = (abc - 100 * a) / 10;
  let c = abc - 100 * a - b * 10;

  let ans = a + b + c;
  println!("{}", ans);
}