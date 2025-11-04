use std::io;

fn main() {
    let mut scale_input = String::new();
    let mut temp_input = String::new();

    println!("Fahrenheit to Celsius (F) or Celsius to Fahrenheit (C)?\n");
    io::stdin().read_line(&mut scale_input).unwrap();

    println!("Enter a temperature:");
    io::stdin().read_line(&mut temp_input).unwrap();
    let temp:f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let converted_temp:f64;

    match scale_input.trim().to_uppercase().as_str() {
        "F" => {converted_temp = (temp - 32.0) * (5.0 / 9.0);
                println!("{} Fahrenheit is {} Celsius", temp, converted_temp);}

        "C" => {converted_temp = (temp * (9.0 / 5.0)) + 32.0;
                println!("{} Celsius is {} Fahrenheit", temp, converted_temp);}
        _ => {
            println!("Wrong input. Enter 'F' or 'C'");
        }
    }
}