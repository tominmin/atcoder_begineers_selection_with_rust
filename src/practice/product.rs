/*
[Question]
AtCoDeer the deer has two positive integers.(a, b)
Determine whether the product of b is even or odd.

[Restriction]
1 ≤ a,b ≤ 10000
a,b are integers
*/
use proconio::input;

pub fn product() {
    input!{
        ab: [i32; 2],
    };
    
    if ab[0] * ab[1] % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}