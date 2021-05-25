struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin(&self) -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

struct Rectangle {
    // p1: Point,
    // p2: Point,
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;
//         // `abs` is a `f64` method that returns the absolute value of the
//         // caller
//         ((x1 - x2) * (y1 - y2)).abs()
//     }
// }

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // fn distance(&self, p1: Point, p2: Point) -> u32 {
    //     if p1.x > { }
    // }
}

#[test]
fn test_area() {
    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };

    assert_eq!(rectangle.area(), 100)
}

#[test]
fn test_square() {
    assert_eq!(Rectangle::square(10).area(), 100)
}

#[test]
fn test_can_hold() {
    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };

    assert_eq!(
        rectangle.can_hold(&Rectangle {
            width: 50,
            height: 100
        }),
        false
    )
}
