fn main(){
    let balance: i32 = 5_000;
    // Scope:
    {
        let balance: i32 = balance + 1;
        println!("{balance}");
    }
    println!("{balance}");
}