use std::io;

fn main() {
    'units_input: loop {
        let mut units = String::new();
        println!("Convert F or C? (Enter F or C)");
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");

        match units.trim() {
            "F" => {
                let final_temp = f_to_c();
                println!("Temp in C is {}", final_temp);
                break 'units_input;
            }
            "C" => {
                let final_temp = c_to_f();
                println!("Temp in F is {}", final_temp);
                break 'units_input;
            }
            _ => continue,
        }
    }
    println!("broken");
}

fn f_to_c() -> i32 {
    // (f - 32) * 5/9
    let _temp = temp_input();
    return (_temp - 32) * 5 / 9;
}

fn c_to_f() -> i32 {
    // (c * 9/5) + 32
    let _temp = temp_input();
    return (_temp * 9 / 5) + 32;
}

fn temp_input() -> i32 {
    // function to gather temperature
    // that the user wants converted
    'temp_input: loop {
        let mut temp = String::new();
        println!("What temperature would you like to convert?");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let trimmed = temp.trim().parse::<i32>().expect("Invalid input");
        return trimmed;
    }
}
