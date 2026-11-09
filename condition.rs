fn main() {
    let balance: i32 = 0;

    if balance > 0 {
        println!("Balance is greater than zero");
    } else {
        println!("It is not");
    }
}

fn main() {
    //let balance: i32 = 000;
    let mut count: i32 = 0;

    loop {
        //println!("It is a infinite loop");
        if count == 9 {
            break;
        }
        count += 1;
        println!("{count}");
    }
}
