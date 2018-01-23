use std::io;

fn main() {
    println!("Convert fahrenheit to celsius!");

    println!("Please input your temperature to convert.");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess:f32 = guess.trim().parse()
            .expect("Failed to read number.");

        let converted:f32 = fahrenheit_to_celsius(guess);

        println!("You input {} Fahrenheit which is {} Celsius", guess, converted);
        }
    }

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.)/1.8
}
