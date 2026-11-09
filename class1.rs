// Print
fn main(){
    println!("Hello");
}

// Variables and Types
const LAMPORTS_PER_SOL: u64 = 1_000_000_000; // u64 for big data

let balance = 5_000; // imutable
let mut total = balance; // can change
total += 1_000;

let total = total * 2; // shadowing

// Functions and Control Flow
fn base_fee(signatures: u64) -> u64 {
    signatures * 5_000 // base fee
}

fn total(amount: &[u64]) -> u64 {
    let mut sum = 0;
    for amount in amounts {sum += amount}
    sum
}

fn find(id: u8) -> Option<u64>