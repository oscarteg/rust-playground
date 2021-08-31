struct HasDrop {
    foo: String,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping: {}", self.foo);
    }
}

struct FireWork {
    strength: i32,
}

impl Drop for FireWork {
    fn drop(&mut self) {
        println!("Dropping: {}", self.strength);
    }
}

#[test]
fn test_drop() {
    let x = HasDrop {
        foo: String::from("bar"),
    };
}
