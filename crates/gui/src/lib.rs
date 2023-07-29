pub trait Draw {
    fn draw(&self) {
        todo!("Implement draw method");
    }
}

pub struct Screen {
    // Box<dyn draw> is a trait object, stand in for any object within box that implements Draw
    pub components: Vec<Box<dyn Draw>>,
}
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

impl Screen {
    // impl<T> Screen<T>
    // where
    //     T: Draw,
    // {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Square;

impl Draw for Square {
    fn draw(&self) {
        println!("I am a square");
    }
}

struct Button {
    pub width: u32,
    pub height: u32,
    pub label: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("I am a button");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //! Can only use 1 type implementation
        //! Uses __static dispatch__ because we KNOW at compile time
        //! Faster runtime because there is a shorter lookup on what function to execute
        //! let mut screen: Screen<Square> = Screen { components: vec![] };
        //! screen.components.push(Square {});

        //! Can implement multiple types that have the draw trait
        //! Uses __dynamic dispatch__ so rust has to lookup what function to execute.
        let mut screen: Screen = Screen { components: vec![] };
        screen.components.push(Box::new(Square {}));

        screen.run();
    }
}
