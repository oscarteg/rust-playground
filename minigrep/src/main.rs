use minigrep::{run, Config};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // --snip--

    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;

    // #[test]
    // fn should_create_config() -> Result<(), Box<dyn std::error::Error>> {
    // let mut cmd = Command::cargo("grrs")?;
    //
    // cmd.arg("foobar").arg("test/file/doesnt/exist");
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("No such file or directory"));
    //
    // Ok(())
    // }
}
