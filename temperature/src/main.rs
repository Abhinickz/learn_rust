use std::io;

fn main() {

    // Take input to choose correct option using this loop.
    loop {
        println!("Enter \"1\" to Convert °F => °C");
        println!("Enter \"2\" to Convert °C => °F");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: i8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You entered a Non-Number: {}", option);
                continue;
            }
        };

        // Match the option and call corresopnding code.
        match option {
            1 | 2 => {
                match option {
                    1 => {
                        f_to_c(1);
                    }
                    2 => {
                        c_to_f(2);
                    }
                    _ => {}
                }
                // all things done, now exit the loop.
                break;
            }
            _ => {
                // In case of other option than 1 or 2 continue with the option input loop.
                continue;
            }
        }
    }
}

fn f_to_c(opt: i8) {
    let temp_f: f32 = input_temperature(opt);
    println!("temp in °F: {}", temp_f);
    println!("temp in °C: {}", (temp_f - 32.0) * 0.5556);
}

fn c_to_f(opt: i8) {
    let temp_c: f32 = input_temperature(opt);
    println!("temp in °C: {}", temp_c);
    println!("temp in °F: {}", (temp_c * 9.0 / 5.0) + 32.0);
}

fn input_temperature(opt: i8) -> f32 {

    // Dynamically generate string based on option.
    let str = match opt {
        1 => "°F",
        2 => "°C",
        _ => "",    // Default case needed.
    };

    let mut temp = String::new();
    println!("Enter temperature in {}:", str);
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You entered a Non-floating number: {}", temp);
            input_temperature(opt) // Recursive call in case of input error for temperature.
        }
    };

    // Instead of using return keyword, temp could be written to return.
    return temp; // temp
}

/*
Output:
[01:45:25 abhasker@wsl -> temperature$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/temperature`
Enter "1" to Convert °F => °C
Enter "2" to Convert °C => °F
2
Enter temperature in °C:
37.0
temp in °C: 37
temp in °C: 98.6
*/
