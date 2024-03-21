use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {:?}", vec![self.x, self.y])
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
    pub(crate) points: Vec<Point>,
    pub(crate) divided: bool
}

impl QuadTree {
    pub fn new (boundary: Rectangle, capacity: usize) -> QuadTree {
        QuadTree {
            boundary: boundary,
            capacity: capacity,
            points: vec![],
            divided: false,  
        }
    }

    pub fn insert(&mut self, point: Point) {
        if self.points.len() < self.capacity {
            self.points.push(point);
        }
        else {
            if !self.divided {
                self.subdivide();
                self.divided = true;
            }
        }
    }

    pub fn subdivide(&mut self) {
        let tl_rec = Rectangle::new(self.boundary.x - self.boundary.w/2.0, self.boundary.y + self.boundary.h/2.0, self.boundary.w/2.0, self.boundary.h/2.0);
        let top_left = QuadTree::new(tl_rec, 0);

        let bl_rec = Rectangle::new(self.boundary.x + self.boundary.w/2.0, self.boundary.y + self.boundary.h/2.0, self.boundary.w/2.0, self.boundary.h/2.0);
        let bottom_left = QuadTree::new(bl_rec, 0);

        let tr_rec = Rectangle::new(self.boundary.x - self.boundary.w/2.0, self.boundary.y - self.boundary.h/2.0, self.boundary.w/2.0, self.boundary.h/2.0);
        let top_right = QuadTree::new(tr_rec, 0);

        let br_rec = Rectangle::new(self.boundary.x + self.boundary.w/2.0, self.boundary.y - self.boundary.h/2.0, self.boundary.w/2.0, self.boundary.h/2.0);
        let bottom_right = QuadTree::new(br_rec, 0);
    }
}

impl fmt::Display for QuadTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "QuadTree: {:?} {:?} {:?}", self.boundary.to_string(), self.capacity, self.points)
    }
}