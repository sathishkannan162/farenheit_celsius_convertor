use std::io::{self, Write};

fn main() {
    println!("Enter the temperature in celsius");
    print!(
        "Enter your choice:
    1. Farenheit to Celsius
    2. Celsius to Farenheit 
    Enter (1 or 2): "
    );
    io::stdout().flush().unwrap();
    let choice = get_int_input();
    let is_valid = verify_choice(choice);
    if is_valid {
        if choice == 1 {
            println!("Convetint Farenheit to Celsius");
            print!("Enter your temperature in Farenheit: ");
            flush_out();
            let f = get_float_input();
            let c = fah_cel(f);
            println!("The given temperature {f} in celsius is {c} ");
        } else if choice == 2 {
            println!("Convetint Celsius to Farenheit");
            print!("Enter your temperature in Celsius: ");
            io::stdout().flush().unwrap();
            let c = get_float_input();
            let f = cel_fah(c);
            println!("The celsius temperature {c} in farenheit is {f} ");
        }
    };
}

fn flush_out() {
    io::stdout().flush().unwrap();
}

fn get_float_input() -> f32 {
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");

    let c: f32 = c.trim().parse().expect("Please enter a decimal number");
    c
}

fn get_int_input() -> i8 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read line");
    let i = i.trim().parse().expect("Enter 1 or 2");
    i
}

fn verify_choice(choice: i8) -> bool {
    if choice == 1 || choice == 2 {
        println!("Your choice is {choice}");
        return true;
    };
    println!("Your input should be 1 or 2");
    return false;
}

fn cel_fah(c: f32) -> f32 {
    let f = c * (9.0 / 5.0) + 32.0;
    f
}

fn fah_cel(f: f32) -> f32 {
    let c = (f - 32.0) * (5.0 / 9.0);
    c
}
