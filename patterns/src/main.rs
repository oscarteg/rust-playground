fn main() {
    let favorite_color: Option<&str> = None;

    let is_tuesday = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("color: {} ", color)
    } else if is_tuesday {
        println!("its tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple")
        } else {
            println!("using ornage")
        }
    } else {
        println!("using blue as the background");
    }

    let mut stack = vec![..5];

    while let Some(top) = stack.pop() {
        println!("{:?}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, val) in v.iter().enumerate() {
        println!("{} at index {} ", val, index)
    }
}
