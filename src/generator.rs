use core::fmt;
use rand::Rng;
use serde_json::json;

use std::fs::File;
use std::io::prelude::Write;

use crate::rectangle::Rectangle;

fn random_dim() -> (u32, u32) {
    let w = rand::thread_rng().gen_range(10..20);
    let h = rand::thread_rng().gen_range(w + 1..50);

    (w, h)
}

pub fn create_base_rectangle() -> Rectangle {
    let (width, height) = random_dim();

    Rectangle::new(0, rand::thread_rng().gen_range(50..100), width, height)
}
pub fn create_ajacent_rectangle(r: &Rectangle) -> Rectangle {
    let (width, height) = random_dim();
    Rectangle::new(r.x + r.width, r.y, width, height)
}

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
                    // left most vertical ajacent to get x coordinates
                    let left_most = higher_ajacent.first().unwrap();

                    // get all rectangles below current
                    let bottom: Vec<&Rectangle> = prev
                        .iter()
                        .filter(|p| p.x <= cur.x && p.x + p.width >= cur.x)
                        .collect();

                    // get last rectangle below current
                    let bottom_rectangle = bottom.last().unwrap();
                    // y coordinates based on bottom rectangle
                    let y: u32 = bottom_rectangle.y - bottom_rectangle.height;
                    let height_to_bottom: u32 = bottom.iter().map(|x| x.height).sum();
                    let width: u32 = higher_ajacent.iter().map(|x| x.width).sum();
                    prev.push(Rectangle::new(
                        left_most.x,
                        y,
                        width,
                        cur.height - height_to_bottom,
                    ));
                } else {
                    // no higher ajacent elements
                    let len = source_rectangles.len();

                    let ajacent = if original_index == 0 {
                        // original index is 0, pick right rectangle
                        &source_rectangles[1]
                    } else if original_index == len - 1 {
                        // original index is last, pick left rectangle
                        &source_rectangles[len - 2]
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
                        ajacent.y - ajacent.height,
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
    use crate::{
        generator::{
            create_ajacent_rectangle, create_base_rectangle, transform_rectangles, Generator,
        },
        rectangle::Rectangle,
    };

    use more_asserts::assert_gt;

    #[test]
    fn test_rectangle_random_ajacent() -> Result<(), ()> {
        let rectangle = create_base_rectangle();
        let ajacent = create_ajacent_rectangle(&rectangle);

        assert_gt!(rectangle.height, rectangle.width);
        assert_gt!(ajacent.height, ajacent.width);
        assert_gt!(rectangle.y, rectangle.height);

        assert_eq!(rectangle.x, 0);
        assert_eq!(ajacent.x, rectangle.x + rectangle.width);
        assert_eq!(rectangle.y, ajacent.y);

        println!("{}", rectangle);
        println!("{}", ajacent);

        Ok(())
    }

    #[test]
    fn test_random_rectangle_transform() -> Result<(), ()> {
        let _ = Generator::new(15);

        Ok(())
    }

    #[test]
    fn test_rectangle_transform() -> Result<(), ()> {
        let source_rectangles: Vec<Rectangle> = serde_json::from_str(
            r#"[
                { "x": 0, "y": 150, "width" : 50, "height": 100},
                { "x": 50, "y": 150, "width" : 40, "height": 87},
                { "x": 90, "y": 150, "width" : 70, "height": 66},
                { "x": 160, "y": 150, "width" : 45, "height": 146},
                { "x": 205, "y": 150, "width" : 30, "height": 54}
            ]"#,
        )
        .unwrap();

        let target_rectangles = transform_rectangles(&source_rectangles);

        assert_eq!(target_rectangles[0], Rectangle::new(0, 150, 235, 54));
        assert_eq!(target_rectangles[1], Rectangle::new(0, 96, 205, 12));
        assert_eq!(target_rectangles[2], Rectangle::new(0, 84, 90, 21));
        assert_eq!(target_rectangles[3], Rectangle::new(0, 63, 50, 13));
        assert_eq!(target_rectangles[4], Rectangle::new(160, 84, 45, 80));

        Ok(())
    }

    #[test]
    fn test_rectangle_transform_2() -> Result<(), ()> {
        let source_rectangles: Vec<Rectangle> = serde_json::from_str(
            r#"[
                {"x": 0, "y": 66, "width": 56, "height": 61 },
                {"x": 56, "y": 66, "width": 18, "height": 41 },
                {"x": 74, "y": 66, "width": 72, "height": 96 },
                {"x": 146, "y": 66, "width": 14, "height": 63 },
                {"x": 160, "y": 66, "width": 82, "height": 92 }
            ]"#,
        )
        .unwrap();

        let target_rectangles = transform_rectangles(&source_rectangles);

        assert_eq!(target_rectangles[0], Rectangle::new(0, 66, 242, 41));
        assert_eq!(target_rectangles[1], Rectangle::new(0, 25, 56, 20));
        assert_eq!(target_rectangles[2], Rectangle::new(74, 25, 168, 22));
        assert_eq!(target_rectangles[3], Rectangle::new(160, 3, 82, 29));
        assert_eq!(target_rectangles[4], Rectangle::new(74, 3, 72, 33));

        Ok(())
    }
}
