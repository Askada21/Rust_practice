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


fn main() {
    let mut count: i32 = 0;

    while count < 10 {
        println!("{count}");
        count += 1;
    }
}

