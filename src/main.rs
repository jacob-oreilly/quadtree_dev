use quadtree::{QuadTree, Rectangle, Point};
use rand::prelude::*;

mod quadtree;

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 200.0;
fn main() {
    let rectangle = Rectangle::new(200.0, 200.0, WIDTH, HEIGHT);
    let mut qt = QuadTree::new(rectangle, 4);
    
    let mut rng = rand::thread_rng();
     // generates a float between 0 and 1
    for _i in 0..qt.capacity {
        let x: f32 = rng.gen_range(0.0..WIDTH);
        let y: f32 = rng.gen_range(0.0..HEIGHT);
        let point = Point::new(x, y);
        qt.insert(point)
    }

    println!("QuatTree: {:?}", qt.to_string());
}
