use std::io;

fn main() {
    println!("Convert fahrenheit to celsius!");

    loop {

        let mut user_input = String::new();

        println!("Please input your temperature to convert.");

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line.");

        if user_input.trim() == "quit" {
            break;
        }

        let temp:f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let converted:f32 = fahrenheit_to_celsius(temp);

        println!("You input {} Fahrenheit which is {} Celsius", temp, converted);
        }
    }

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.)/1.8
}
