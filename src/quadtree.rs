use std::fmt;

pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub fn new (x: f32, y: f32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

pub struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32
}

impl Rectangle {
    pub fn new (x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle xy Coord: {:?}, Rectangle Width/Height: {:?}", vec![self.x, self.y], vec![self.w, self.h])
    }
}

pub struct QuadTree {
    pub(crate) boundary: Rectangle,
    pub(crate) capacity: usize,
    pub(crate) points: Vec<Point>
}

impl QuadTree {
    pub fn new (boundary: Rectangle, capacity: usize) -> QuadTree {
        QuadTree {
            boundary: boundary,
            capacity: capacity,
            points: vec![]    
        }
    }

    pub fn insert(&mut self, point: Point) {
        if self.points.len() < self.capacity {
            self.points.push(point);
        }
    }
}

impl fmt::Display for QuadTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QuadTree: {}", self.boundary.to_string())
    }
}