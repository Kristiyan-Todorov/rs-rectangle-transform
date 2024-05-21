use generator::Generator;
use serde_json::json;
use std::io::stdin;

pub mod generator;
pub mod rectangle;

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

    let generator = Generator::new(number_of_rectangles);

    println!("{}", generator);
}

// let source_rectangles: Vec<Rectangle> = serde_json::from_str(
//     r#"[{ "x": 0, "y": 150, "width" : 50, "height": 100}, { "x": 50, "y": 150, "width" : 40, "height": 87}, { "x": 90, "y": 150, "width" : 70, "height": 66}, { "x": 160, "y": 150, "width" : 45, "height": 146},{ "x": 205, "y": 150, "width" : 30, "height": 54}]"#,
// ).unwrap();
