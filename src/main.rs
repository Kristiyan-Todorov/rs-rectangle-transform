use generator::Generator;
use std::io::stdin;

pub mod generator;
pub mod rectangle;

fn main() {
    let mut s = String::new();
    let number_of_rectangles: usize;

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

    let generator = Generator::new(number_of_rectangles);

    println!("{}", generator);

    generator
        .write_file("rectangle_transform.json")
        .expect("Unable to write to file");
}
