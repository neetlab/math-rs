use std::ops::Add;
use std::f64;

# [derive(Debug, PartialEq, PartialOrd)]
struct Vector(f64, f64);

impl Vector {
    fn norm(&self) -> f64 {
        let Vector(x, y) = self;
        return x.hypot(*y);
    }

    fn multiply(&self, k: f64) -> Vector {
        let Vector(x1, y1) = self;
        return Vector(x1*k, y1*k);
    }

    fn angle(&self, other: &Vector) -> f64 {
        return (self.dot_prod(other) / (self.norm() * other.norm())).acos();
    }

    fn dot_prod(&self, other: &Vector) -> f64 {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = other;
        return x1 * x2 + y1 * y2;
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = other;
        return Vector(x1 + x2, y1 + y2);
    }
}

fn main() {
    let v1 = Vector(1.0, 2.0);
    let v2 = Vector(2.0, 1.0);
    let new_vec = &v1 + &v2;
    println!("norm1 {:?}", v1.norm());
    println!("norm2 {:?}", v2.norm());
    println!("sum  {:?}", new_vec);
    println!("len  {:?}", new_vec.norm());
    println!("prod {:?}", v1.dot_prod(&v2));
    println!("ang  {:?}", v1.angle(&v2));
}
