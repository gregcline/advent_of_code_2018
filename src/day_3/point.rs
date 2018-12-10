use std::fmt;
use std::iter::Iterator;

#[derive(Debug, Hash, Eq, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    id: usize,
    point: Point,
    width: usize,
    height: usize,
}

impl Rectangle {
    pub fn new(id: usize, x: usize, y: usize, width: usize, height: usize) -> Self {
        Rectangle {
            id: id,
            point: Point { x: x, y: y },
            width: width,
            height: height,
        }
    }

    pub fn points(&self) -> RectPoints {
        RectPoints {
            current: self.point.clone(),
            left: self.point.x,
            right: self.point.x + self.width - 1,
            bottom: self.point.y + self.height - 1,
        }
    }
}

#[derive(Debug)]
pub struct RectPoints {
    current: Point,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Iterator for RectPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.y > self.bottom {
            None
        } else if self.current.x == self.right {
            let curr = self.current.clone();
            self.current.x = self.left;
            self.current.y += 1;
            Some(curr)
        } else {
            let curr = self.current.clone();
            self.current.x += 1;
            Some(curr)
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{} @ {},{}: {}x{}",
            self.id, self.point.x, self.point.y, self.width, self.height
        )
    }
}
