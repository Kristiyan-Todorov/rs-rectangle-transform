use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

static LOW: u32 = 10;
static HIGH: u32 = 100;

fn random() -> u32 {
    rand::thread_rng().gen_range(LOW..HIGH)
}

fn random_vertical_dim() -> (u32, u32) {
    let w = random();
    // ensure vertical in case width equals HIGH
    let h = rand::thread_rng().gen_range(w..HIGH + 1);

    (w, h)
}

pub fn create_base_rectangle() -> Rectangle {
    let (width, height) = random_vertical_dim();

    Rectangle::new(0, random(), width, height)
}
pub fn create_ajacent_rectangle(r: &Rectangle) -> Rectangle {
    let (width, height) = random_vertical_dim();
    Rectangle::new(r.x + r.width, r.y, width, height)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width,
            height,
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(x:{}, y:{}, width:{}, height: {})",
            self.x, self.y, self.width, self.height
        )
    }
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_gt;

    use crate::rectangle::{create_ajacent_rectangle, create_base_rectangle};

    #[test]
    fn test_rectangle_random_ajacent() -> Result<(), ()> {
        let rectangle = create_base_rectangle();
        let ajacent = create_ajacent_rectangle(&rectangle);

        assert_gt!(rectangle.height, rectangle.width);
        assert_gt!(ajacent.height, ajacent.width);

        assert_eq!(rectangle.x, 0);
        assert_eq!(ajacent.x, rectangle.x + rectangle.width);
        assert_eq!(rectangle.y, ajacent.y);

        println!("{}", rectangle);
        println!("{}", ajacent);

        Ok(())
    }
}
