mod structs;
mod traits;

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<String, u64> = HashMap::new();
    let test = String::from("test");
    values.insert(test, 12345);
    println!("\"test\" associated value is {}", values.get("testa").unwrap());
}

// Exercise 3
// deposit function takes 2 args, a Context<Deposit> and u64
// I suppose ctx is some piece of state
// and collat is how much token depositor wants to deposit
// let rate is some form of sload, or view function to get a rate in the form of u128
// let amt is amount of shares to mint to Depositor by colalat * rate / DECIMALS?SCALAR
// transfer collat amout tokens from caller to this contract
// mint amt amount of shares_token to caller
// OK

// Questions:

// what is token::?
// what does the `?` at the end mean? was the transfer even successful? mint successful?

// ok so exchange_rate.deposit_rate is an u64.
// that means we upcast it - should be ok 
// DECIMALS_SCALAR value is not given.
//  It is an u128 which may have any value except 
// but we downcast it.
// if DECIMALS_SCALAR > max u64 ,
// It is possible that collat * rate / Decimals_Scalar> maxU64
// where collat * rate / Decimals_Scalar can be 2^128 - 1 
// and decimals scalar is 1
// then we downcast amt as u64 to get lower