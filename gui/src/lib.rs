pub trait Draw {
    fn draw(&self) {
        todo!("Implement draw method");
    }
}

pub struct Screen {
    // Box<dyn draw> is a trait object, stand in for any object within box that implements Draw
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut screen: Screen = Screen { components: vec![] };
        screen.components.push(Box::new(Square {}));

        screen.run();
    }
}
