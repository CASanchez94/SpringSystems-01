fn check_guess(guess: i32, secret:i32) -> i32{
if guess == secret{
    0
} else if guess > secret{
    1
} else{
    -1
}

}

fn main() {
let secret_number: i32 = 5;
let mut guess_taken: i32 = 0;
let mut guess: i32 = 1;



loop {

    guess_taken += 1;

    let check = check_guess(guess, secret_number);

    if check == 0{
        println!("Guess {} is correct", guess);
        break;
    } else if check == 1 {
        println!("Guess {} is too high", guess);
    } else {

        println!("Guess {} is too low", guess);
    }
    guess += 1;
}

println!{"Guesses Taken {}", guess_taken};


}
