use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

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
