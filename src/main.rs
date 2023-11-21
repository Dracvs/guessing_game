use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

fn main() {
    
    println!("
        1. Guessing Game 
        2. Variable Mutability
        3. Shadowing
        4. Arrays
        5. Functions
        6. More Functions
        7. Return Value Functions
        8. Loops!
        9. The Farehnheit Anxiom
        10. the Celsius Problem
        11. Borrow Checker Princples
        12. String Slices
        13. Actual String Slices Implementation
        14. Pointer Sizes");

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
    else if activity.trim().eq_ignore_ascii_case("5")
    {
        functions();
    }
    else if activity.trim().eq_ignore_ascii_case("6")
    {
        more_functions();
    }
    else if activity.trim().eq_ignore_ascii_case("7")
    {
        return_value_functions();
    }
    else if activity.trim().eq_ignore_ascii_case("8")
    {
        loops();
    }
    else if activity.trim().eq_ignore_ascii_case("9")
    {
        from_fahrenheit_to_celsius();
    }
    else if activity.trim().eq_ignore_ascii_case("10")
    {
        from_celsius_to_fahrenheit();
    }
    else if activity.trim().eq_ignore_ascii_case("11")
    {
        borrow_checker();
    }
    else if activity.trim().eq_ignore_ascii_case("12")
    {
        string_slices();
    }
    else if activity.trim().eq_ignore_ascii_case("13")
    {
        let salute: String = String::from("Royal Salute!");
	    // `string_slices_actual_impl` works on slices of 
        // `String`s whether partial or whole

        let word = string_slices_actual_impl(&salute[0..6]);
        let word = string_slices_actual_impl(&salute[..]);

        // `string_slices_actual_impl` also works on references to `String`
        // which are equivalente to whole slices of `String`s
        let word = string_slices_actual_impl(&salute);

        let salute_literal = "Royal Salutation!";

        // `string_slices_actual_impl` works on slices of string literals
        // whether partial or whole
        let word = string_slices_actual_impl(&salute_literal[0..6]);
        let word = string_slices_actual_impl(&salute_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!

        string_slices_actual_impl(salute_literal);
    }
    else if activity.trim().eq_ignore_ascii_case("14") {
        check_pointer_sizes();
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

fn functions()
{
    println!("gimme a number");
    let mut number = String::new();
    
    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line"); 

    let number: i32 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };

    another_function(number);
}

fn another_function(x : i32)
{
    println!("The passed value of x is {x}");
}

fn more_functions()
{
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char)
{
    println!("The Measurement is: {value}{unit_label}");
}

fn return_value_functions()
{
    println!("The returnal value functons is {}", returnal_value_function());
}

fn returnal_value_function() -> i32
{
    return rand::thread_rng().gen_range(1..=10);
}

fn loops()
{

}

fn borrow_checker()
{
    let m1 = String::from("The World");
    let m2 = String::from(" is on fire");
    greet(&m1, &m2);
    let _s = format!("{} {}!", m1, m2);
}

fn greet(g1: &String, g2: &String)
{
    println!("{}{}", g1, g2);
}

fn from_fahrenheit_to_celsius()
{
    println!("Input Fahrenheit degrees: ");
    let fahrenheit = capture_input();
    let fahrenheit = string_to_float(fahrenheit);
    let celsius  = (fahrenheit - 32.0) * 5.0/9.0;
    println!("{fahrenheit} fahrenheit makes {celsius} celsius");
}

fn from_celsius_to_fahrenheit()
{
    println!("Input Celcius degrees: ");

    let celsius = capture_input();

    let celsius = string_to_float(celsius);

    let fahrenheit = (celsius * 9.0/5.0) + 32.0;
    println!("{celsius} celsius makes {fahrenheit} fahrenheit");
}


fn capture_input() -> String
{
    let mut input_text = String::new(); 
    io::stdin()
        .read_line(&mut input_text)
        .expect("Could not read line");
    return input_text;
}

fn string_to_integer(text: String) -> i32
{
    let number = match text.trim().parse(){
        Ok(num)=> num,
        Err(_) => 0  
    };
    return number;
}

fn string_to_float(text: String)->f32
{
    let number = match text.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0
    };
    return number;
}

fn string_slices()
{
    let mut s = String::from("hello");
    let hello: &str = &s[0..5];  
    // This won't compile if placed here: 
    //s.push_str(" World");  
    println!("{hello}");
    s.push_str(" World");
    
}

fn string_slices_actual_impl(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn check_pointer_sizes()
{
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>()
    )
}
