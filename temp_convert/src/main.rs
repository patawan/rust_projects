use std::io;

fn main() {
    'input: loop {
        let mut units = String::new();
        println!("Convert F or C? (Enter F or C)");
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");

        match units.trim() {
            "F" => {
                f_to_c();
                break 'input;
            }
            "C" => {
                c_to_f();
                break 'input;
            }
            _ => continue,
        }
    }
    println!("broken");
}

fn f_to_c() {
    println!("f to c");
}

fn c_to_f() {
    println!("c to f");
}
