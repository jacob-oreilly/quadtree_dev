use quadtree::{QuadTree, Rectangle, Point};

mod quadtree;
fn main() {
    let rectangle = Rectangle::new(200.0, 200.0, 200.0, 200.0);
    let mut qt = QuadTree::new(rectangle, 4);
    
    for i in 0..1 {
        let point = Point::new(200.0, 200.0);
        qt.insert(point)
    }

    println!("QuatTree: {:?}", qt.to_string());
}
