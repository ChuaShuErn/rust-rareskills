
pub trait AdditiveOperations {
    fn add( a: i64 , b: i64) -> i64{
        return a + b;
    }

    fn sub( a: i64, b:i64) ->i64{

        return a-b;
    }
}

pub trait MultiplicativeOperations {

    fn mul (a: i64, b:i64) -> i64 {
        return a * b;
    }

    fn div (a: i64, b:i64) ->i64{
        if(b==0){
            return 0;
        }
        return a/b;
    }

}

pub trait BinaryOperations{
    fn and (a :i64, b:i64) -> i64 {

        return a&b;

    }
    fn or (a :i64, b:i64) -> i64 {
        return a|b;
    }
    fn xor (a :i64, b:i64) -> i64 {
        return a^b;
    }
}

pub trait CalculatorSuperTrait:AdditiveOperations+ MultiplicativeOperations +BinaryOperations{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, "Calculator Print entered, inputs are {} and {}", self.first, self.second);
        write(f, "Adding {} and {} to give {}", self.first, self.second, add)
    }
}