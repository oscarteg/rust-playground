struct Counter {}

enum Medewerker<'a> {
    Oscar(i32),
    Rune(&'a str),
}

fn foo(m: &Medewerker) {
    match m {
        Medewerker::Oscar(i) => {
            println!("Oscar is {}", i);
        }
        Medewerker::Rune(s) => {
            println!("Rune heeft geen getal maar wel: {}", s);
        }
    }

    ()
}
