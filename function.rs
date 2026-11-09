pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {
    let x: i32 = 1;
    let y: i32 = 2;

    let ans: i32 = sum(x, y);
    println!("{ans}");
}