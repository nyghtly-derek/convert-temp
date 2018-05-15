use std::io;

fn main() {
    println!("This program will convert temperature between F and C.");

    let temp_type = get_temp_type();
    let temp = get_temp();

    println!("Your temperature is {}{}", temp, temp_type);
}

fn get_temp_type() -> String {
    loop {
        println!("What is your temperature type? (f/c)");
        
        let temp_type = get_line();
        let temp_type: String = temp_type.trim().to_string();

        match temp_type.as_ref() {
            "f" => return temp_type,
            "c" => return temp_type,
            "C" => return temp_type,
            "F" => return temp_type,
            _ => println!("That is not a valid temp type, please try again."),
        }
    }
}

fn get_temp() -> u32 {
     loop {
        println!("What is your temperature?");

        let temp = get_line();

        let temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temp
    }
}

fn get_line() -> String {
    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .expect("Failed to read line");

    return line
}


