use core::fmt;
use std::fs::File;
use std::io::prelude::Write;

use serde_json::json;

use crate::rectangle::{create_ajacent_rectangle, create_base_rectangle, Rectangle};

fn generate_rectangles(number_of_rectangles: usize) -> Vec<Rectangle> {
    std::iter::repeat(())
        .take(number_of_rectangles)
        .enumerate()
        .fold(Vec::new(), |mut prev, (idx, _)| {
            if idx == 0 {
                prev.push(create_base_rectangle())
            } else {
                prev.push(create_ajacent_rectangle(&prev[idx - 1]))
            }
            prev
        })
}

fn transform_rectangles(source_rectangles: &Vec<Rectangle>) -> Vec<Rectangle> {
    let mut height_sorted = source_rectangles.clone();
    height_sorted.sort_by(|a, b| a.height.cmp(&b.height));

    height_sorted
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut prev, (idx, cur)| {
            if idx == 0 {
                // Handle the base rectangle separately
                let right_most = source_rectangles.last().unwrap();
                prev.push(Rectangle::new(
                    0,
                    cur.y,
                    right_most.x + right_most.width,
                    cur.height,
                ));
            } else {
                let original_index = source_rectangles.iter().position(|r| r == cur).unwrap();
                // look left for higher ajacent rectangles
                let mut left: Vec<Rectangle> = Vec::new();
                for i in (0..original_index).rev() {
                    if source_rectangles[i].height < cur.height {
                        break;
                    }
                    left.push(source_rectangles[i].clone())
                }

                let left: Vec<&Rectangle> = left.iter().rev().collect();

                // look right for higher ajacent rectangles
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
                // merge ajacent rectangles and include current
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
                    // no higher ajacent elements
                    let len = source_rectangles.len();

                    let ajacent = if original_index == 0 {
                        // original index is 0, pick right rectangle
                        &source_rectangles[1]
                    } else if original_index == len {
                        // original index is last, pick left rectangle
                        &source_rectangles[len - 1]
                    } else {
                        // pick higher ajacent rectangle
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

pub struct Generator {
    number_of_rectangles: usize,
    source_rectangles: Vec<Rectangle>,
    target_rectangles: Vec<Rectangle>,
}

impl Generator {
    pub fn new(number_of_rectangles: usize) -> Generator {
        let source_rectangles = generate_rectangles(number_of_rectangles);
        let target_rectangles = transform_rectangles(&source_rectangles);

        Generator {
            number_of_rectangles,
            source_rectangles,
            target_rectangles,
        }
    }

    pub fn write_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.generate_json().as_bytes())?;
        Ok(())
    }

    fn generate_json(&self) -> String {
        serde_json::to_string_pretty(&json!({
            "numRects": self.number_of_rectangles,
            "sourceRectangles": self.source_rectangles,
            "targetRectangles": self.target_rectangles
        }))
        .unwrap()
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generate_json())
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;

    use super::transform_rectangles;

    #[test]
    fn test_rectangle_transform() -> Result<(), ()> {
        let source_rectangles: Vec<Rectangle> = serde_json::from_str(
            r#"[{ "x": 0, "y": 150, "width" : 50, "height": 100}, { "x": 50, "y": 150, "width" : 40, "height": 87}, { "x": 90, "y": 150, "width" : 70, "height": 66}, { "x": 160, "y": 150, "width" : 45, "height": 146},{ "x": 205, "y": 150, "width" : 30, "height": 54}]"#,
        ).unwrap();

        let target_rectangles = transform_rectangles(&source_rectangles);

        assert_eq!(target_rectangles[0], Rectangle::new(0, 150, 235, 54));
        assert_eq!(target_rectangles[1], Rectangle::new(0, 54, 205, 12));
        assert_eq!(target_rectangles[2], Rectangle::new(0, 66, 90, 21));
        assert_eq!(target_rectangles[3], Rectangle::new(0, 87, 50, 13));
        assert_eq!(target_rectangles[4], Rectangle::new(160, 66, 45, 80));

        Ok(())
    }
}
