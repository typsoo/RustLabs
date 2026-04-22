mod vec2d;

use vec2d::Vec2D;

fn main() {
    let v1 = Vec2D::default();

    let v2 = Vec2D::new(1.1, 2.2);

    let v3: Vec2D = &v1 + &v2;
    let v4: Vec2D = v1 + v2;

    println!("Vector v1: {}", v1);
    println!("Vector v2: {}", v2);
    println!("Vector v2: {}", v3);
}
