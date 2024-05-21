use std::io::stdin;

fn main() {
    let mut s = String::new();
    let mut number_of_rectangles: usize = 0;

    loop {
        s.clear();
        println!("Enter a number of rectangles (5-15): ");
        stdin().read_line(&mut s).expect("Failed to read line");

        s = s.trim().to_string();

        let v = s.parse::<usize>();

        match v {
            Ok(v) => {
                if v >= 5 && v <= 15 {
                    number_of_rectangles = v;
                    break;
                }
            }
            Err(_) => {}
        }

        println!("Invalid input: {}", s)
    }

    println!("Number of rectangles: {}", number_of_rectangles);
}
