fn main(){
    let balance: i32 = 000;
    let number: i32 = 10;
    {
        println!("{}", number);
        println!("{}", balance);
    }
    println!("{}", number);
}