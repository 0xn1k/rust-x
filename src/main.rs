use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn basic_wokring(){

    println!("Enter a number:");
    let mut input1   = String::new();
    let mut input2  = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let first_number : i32 = input1.trim().parse().expect("Please type a number!");
    let second_number : i32 = input2.trim().parse().expect("Please type a number!");
    let sum = first_number + second_number;
    println!("The sum of {} and {} is {}", first_number, second_number, sum);
    println!("Selecting a random number between 1 and 100...");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
     match first_number.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    let mut x : i32 = 0;
    loop{
        x += 1;
        if x == 10 {
            println!("x is equal to 10, breaking the loop.");
            break;
        }
        println!("x is: {}", x);
    }

}


fn guessing_game(){
    println!("Enter the secret number between 1 and 100:");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input  you guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");
        let x : i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match x.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),       
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },  
        }
    }



}

fn trail(){

    let mut val = String:: new();
    io::stdin().read_line(&mut val).expect("failed");
    let val : i32 = val.trim().parse().expect("Please type a number!");
    let result = if val < 5 {
        "less than 5"
    } else if val == 5 {
        "equal to 5"
    } else {
        "greater than 5"
    };
    println!("The value is {}", result);
    
}

fn shadowing_principles(){
    let x = 5;
    let x = x + 1;
    println!("The value of x in the inner scope is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}





fn main() {
    const MAX_POINTS: u32 = 1000;
    let x = 10;
    println!("the number is {}", x);
    // immutable variable
    let mut y = 5;
    println!("the number is {}", y);
    y = 6;
    println!("the number is {}", y);
    // mutable variable
    println!("The maximum points is {}", MAX_POINTS);

    shadowing_principles();



    

    

    

}
