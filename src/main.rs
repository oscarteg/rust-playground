mod rectangle;
mod structs;

fn main() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..s.len()];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn test_first_word() {
    assert_eq!(first_word(&String::from("Hello world")), 5)
}

#[test]
fn test_first_word_bug() {
    let mut s = String::from("Hello world");
    let word = first_word(&s); // Value will be 5

    s.clear(); // Empties string makes it equal to ""
    assert_eq!(word, 5);

    // Word still has 5 but the string has been set to 5
}

#[test]
fn test_first_word_slice() {
    let s = String::from("hello world");

    let word = first_word_slice(&s);

    let s2 = "hello world";

    let word = first_word_slice(s2);

    // s.clear(); // error!

    println!("{}", word)
}
