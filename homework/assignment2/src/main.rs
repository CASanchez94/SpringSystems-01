fn is_even(n: i32) -> bool {
    n % 2 == 0
}



fn main(){

let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// FizzBuzz O/E Check
 for num in numbers {
       
        let pattern = (num % 3 == 0, num % 5 == 0, is_even(num));

        let result: &str;
        if pattern.0 && pattern.1 {
            result = "FizzBuzz";   
        } else if pattern.0 {
            result = "Fizz";      
        } else if pattern.1 {
            result = "Buzz";       
        } else if pattern.2 {
            result = "Even";       
        } else {
            result = "Odd";        
        }

    println!("{}: {}", num, result);
}

//while loop

let mut sum = 0;
let mut i = 0;
while i < numbers.len(){
    sum += numbers[i];
    i += 1;
}
println!("Sum of all numbers: {}", sum);


//finds the largest number
let mut largest = numbers[0];
let mut j = 1;

loop {
    if j >= numbers.len(){
        break;
    }
    if numbers[j] > largest{
        largest = numbers[j];
    }
     j += 1;
}

println!{"Largest Number: {}", largest};




}