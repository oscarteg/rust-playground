fn main() {
    let mut list: Vec<Box<dyn Animal>> = Vec::new();

    list.push(Box::new(Herbivore));
    list.push(Box::new(Carnivore));

    for animal in &list {
        animal.eat();
    }
}

trait Animal {
    fn eat(&self);
}

struct Herbivore;
struct Carnivore;

impl Animal for Herbivore {
    fn eat(&self) {
        println!("i eat plants")
    }
}

impl Animal for Carnivore {
    fn eat(&self) {
        println!("i eat meat")
    }
}
