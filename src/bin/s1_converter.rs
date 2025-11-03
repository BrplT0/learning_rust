use std::io;

fn main() {
    let mut scale_input = String::new();
    let mut temp_input = String::new();

    println!("Fahrenheit to Celsius (F)\n or\n Celsius to Fahrenheit (C)?");
    io::stdin().read_line(&mut scale_input).unwrap();

    println!("Enter a temperature:");
    io::stdin().read_line(&mut temp_input).unwrap();
    let temp:f64 = temp_input.trim().parse().unwrap();
    let converted_temp:f64;

    if scale_input.trim().to_uppercase() == "F" {
        converted_temp = (temp - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit is {} Celsius", temp, converted_temp);
    }
    else if scale_input.trim().to_uppercase() == "C" {
        converted_temp = (temp * 9.0 / 5.0) + 32.0;
        println!("{} Celsius is {} Fahrenheit", temp, converted_temp);
    }
    else{
        println!("Wrong input. Next time write (C) or (F)!!!");
    }
}