use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    println!("Choose Activity: ");

    let mut activity = String::new();

    io::stdin()
        .read_line(&mut activity)
        .expect("Failed to read line");

    if activity.trim().eq_ignore_ascii_case("1")
    {
        guessing_game();
    }
    else if activity.trim().eq_ignore_ascii_case("2") 
    {
        variable_mutability();   
    }
    else if activity.trim().eq_ignore_ascii_case("3")
    {
        shadowing();
    }
    else if activity.trim().eq_ignore_ascii_case("4")
    {
        arrays();
    }
}

fn guessing_game()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    //println!("The secret number is: {secret_number}");
    
    loop { 
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq_ignore_ascii_case("quit")
        {
            break;
        }

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

fn variable_mutability()
{
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    println!("The value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
    println!("Second in three hours: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing()
{
    let x = 5;
    let x = x + 1;

    {
        let x = x*2;
        println!("The value of X in the inner scope is: {x}");

    }

    println!("The value of x is: {x}");

}

fn arrays()
{
    let a = [22, 78, 42, 71, 99];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
