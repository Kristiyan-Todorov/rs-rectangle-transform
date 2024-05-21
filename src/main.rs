use std::io::stdin;

use rectangle::Rectangle;

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

    let source_rectangles = std::iter::repeat(())
        .take(number_of_rectangles)
        .enumerate()
        .fold(Vec::new(), |mut prev, (idx, _)| {
            if idx == 0 {
                prev.push(Rectangle::base_vertical())
            } else {
                prev.push(Rectangle::ajacent_vertical(&prev[idx - 1]))
            }
            prev
        });
}
