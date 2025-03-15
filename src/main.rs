
mod structs;
use structs::Calculator;
mod traits;
use traits::CalculatorSuperTrait;
use traits::AdditiveOperations;
use traits::MultiplicativeOperations;
use traits::BinaryOperations;

impl AdditiveOperations for Calculator{}
impl MultiplicativeOperations for Calculator{}
impl BinaryOperations for Calculator{}

impl CalculatorSuperTrait for Calculator {
    fn get_first(&self) -> i64 {
        return self.first;
    }
    
    fn get_second(&self)->i64 {
        return self.second
    }
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First: {}, Second: {}\n", self.first, self.second)?;
       // write!(f, "First is {} is Second is {}", self.first, self.second);
       write!(f, "ADD: {}", <Calculator as AdditiveOperations>::add(self.first, self.second))?;
        // add
        
        write!(f, "SUB: {}\n", <Calculator as AdditiveOperations>::sub(self.first,self.second))?;

        // sub
         write!(f, "MUL: {}\n", <Calculator as MultiplicativeOperations>::mul(self.first,self.second))?;
        // mul
        write!(f, "DIV: {}\n", <Calculator as MultiplicativeOperations>::div(self.first,self.second))?;
        // div
        write!(f, "AND: {}\n", <Calculator as BinaryOperations>::and(self.first,self.second))?;
        // and
        write!(f, "OR: {}\n", <Calculator as BinaryOperations>::or(self.first,self.second))?;
        // or
        write!(f, "XOR: {}\n", <Calculator as BinaryOperations>::xor(self.first,self.second))?;
        // xor
        Ok(())
    }

}
// Implement std::fmt::Display for Calculator
impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as CalculatorSuperTrait>::fmt(self, f)
    }
}

fn main() {
    let calculator = Calculator {
        first: 5,
        second: 3
    };
    println!("calculator: {}", calculator); 
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