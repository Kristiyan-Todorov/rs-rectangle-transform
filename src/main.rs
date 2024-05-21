use std::io::stdin;

use rectangle::Rectangle;
use serde_json::json;

pub mod rectangle;

fn transform_rectangles(source_rectangles: &Vec<Rectangle>) -> Vec<Rectangle> {
    let mut height_sorted = source_rectangles.clone();
    height_sorted.sort_by(|a, b| a.height.cmp(&b.height));

    height_sorted
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut prev, (idx, cur)| {
            if idx == 0 {
                let right_most = source_rectangles.last().unwrap();
                prev.push(Rectangle::new(
                    0,
                    cur.y,
                    right_most.x + right_most.width,
                    cur.height,
                ));
            } else {
                let original_index = source_rectangles.iter().position(|r| r == cur).unwrap();
                // look left
                let mut left: Vec<Rectangle> = Vec::new();
                for i in (0..original_index).rev() {
                    if i == original_index {
                        continue;
                    }
                    if source_rectangles[i].height < cur.height {
                        break;
                    }
                    left.push(source_rectangles[i].clone())
                }

                let left: Vec<&Rectangle> = left.iter().rev().collect();

                // look right
                let mut right: Vec<&Rectangle> = Vec::new();
                for i in original_index..source_rectangles.len() {
                    if i == original_index {
                        continue;
                    }
                    if source_rectangles[i].height < cur.height {
                        break;
                    }
                    right.push(&source_rectangles[i])
                }

                let higher_ajacent = [left, vec![cur], right].concat();
                if higher_ajacent.len() > 1 {
                    let first = higher_ajacent.first().unwrap();
                    let last = higher_ajacent.last().unwrap();
                    let y = prev.iter().map(|r| r.height).sum();
                    prev.push(Rectangle::new(
                        first.x,
                        y,
                        last.x + last.width,
                        cur.height - y,
                    ));
                } else {
                    let len = source_rectangles.len();

                    let ajacent = if original_index == 0 {
                        &source_rectangles[1]
                    } else if original_index == len {
                        &source_rectangles[len - 1]
                    } else {
                        let mut ajacent = [
                            &source_rectangles[original_index - 1],
                            &source_rectangles[original_index + 1],
                        ];
                        ajacent.sort_by(|a, b| a.height.cmp(&b.height));

                        ajacent[1]
                    };

                    prev.push(Rectangle::new(
                        cur.x,
                        ajacent.height,
                        cur.width,
                        cur.height - ajacent.height,
                    ));
                }
            }

            prev
        })
}

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
    let transformed_rectangles = transform_rectangles(&source_rectangles);

    println!(
        "{}",
        serde_json::to_string_pretty(&json!(transformed_rectangles)).unwrap()
    );
}

// let source_rectangles: Vec<Rectangle> = serde_json::from_str(
//     r#"[{ "x": 0, "y": 150, "width" : 50, "height": 100}, { "x": 50, "y": 150, "width" : 40, "height": 87}, { "x": 90, "y": 150, "width" : 70, "height": 66}, { "x": 160, "y": 150, "width" : 45, "height": 146},{ "x": 205, "y": 150, "width" : 30, "height": 54}]"#,
// ).unwrap();
