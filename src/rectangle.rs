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
#[derive(Clone, Serialize, Deserialize, PartialEq)]
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
    pub fn base_vertical() -> Rectangle {
        let (width, height) = random_vertical_dim();

        Rectangle {
            x: 0,
            y: random(),
            width,
            height,
        }
    }
    pub fn ajacent_vertical(r: &Rectangle) -> Rectangle {
        let (width, height) = random_vertical_dim();
        Rectangle {
            x: r.x + r.width,
            y: r.y,
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

    use super::Rectangle;

    #[test]
    fn test_rectangle_random_ajacent() -> Result<(), ()> {
        let rectangle = Rectangle::base_vertical();
        let ajacent = Rectangle::ajacent_vertical(&rectangle);

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
