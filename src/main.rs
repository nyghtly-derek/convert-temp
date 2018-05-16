use std::io;

fn main() {
    println!("This program will convert temperature between F and C.");

    let temp_type = get_temp_type();
    let temp = get_temp();
    let mut temp_convert: f32 = 0.0;
    let mut temp_convert_type = "";

    if temp_type == "f" || temp_type == "F" {
        temp_convert = far_to_celsius(temp);
        temp_convert_type = "C";
    }
    else if temp_type == "c" || temp_type == "C" {
        temp_convert = cel_to_fahren(temp);
        temp_convert_type = "F";
    }

    println!("{}°{} = {}°{}", temp, temp_type, temp_convert, temp_convert_type);
}

fn far_to_celsius(temp: f32) -> f32 {
    return (temp - 32.0) / 1.8
}

fn cel_to_fahren(temp: f32) -> f32 {
    return temp * 1.8 + 32.0
}

fn get_temp_type() -> String {
    loop {
        println!("What is your temperature type? (F/C)");
        
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

fn get_temp() -> f32 {
     loop {
        println!("What is your temperature?");

        let temp = get_line();

        let temp: f32 = match temp.trim().parse() {
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


