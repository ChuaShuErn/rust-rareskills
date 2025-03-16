
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
       write!(f, "ADD: {}\n", <Calculator as AdditiveOperations>::add(self.first, self.second))?;
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
  // 1 arg
fn print_output<T:CalculatorSuperTrait>(input : &T) {
    let first = input.get_first();
    let second = input.get_second();
    println!("Add: {}", <T as AdditiveOperations>::add(first, second));
    println!("Sub: {}", <T as AdditiveOperations>::sub(first, second));
    println!("Mul: {}", <T as MultiplicativeOperations>::mul(first, second));
    println!("Div: {}", <T as MultiplicativeOperations>::div(first, second));
    println!("And: {}", <T as BinaryOperations>::and(first, second));
    println!("Or: {}", <T as BinaryOperations>::or(first, second));
    println!("Xor: {}", <T as BinaryOperations>::xor(first, second));

}
// Implement std::fmt::Display for Calculator
impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as CalculatorSuperTrait>::fmt(self, f)
    }
}

fn main() {
    let calculator = Calculator {
        first: 1,
        second: 2
    };
    println!("calculator: {}", calculator); 
    print_output(&calculator);
}
