use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

fn main() {
    let mut x = vec![1, 2, 3];

    let y = vec![4, 5];

    let mut merge_vec = |z: Vec<i32>| x.extend(z);

    merge_vec(y);

    assert_eq!(x, vec![1, 2, 3, 4]);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> &mut u32 {
        // match self.value {
        //     Some(value) => value,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
        self.value.entry(arg).or_insert((self.calculation)(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_value() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(500);
        let v2 = c.value(1000);

        assert_eq!(*v2, 1000)
    }
}
