use std::io::{Write, stdin, stdout};

pub fn get_input<T: Clone>(prompt: &str, options: &[(&str, T)]) -> Option<T> {
    let mut input = String::new();

    print!("{}", prompt);

    loop {
        input.clear();
        stdout().flush().ok()?;
        stdin().read_line(&mut input).ok()?;
        let trimmed = input.trim();

        for (pattern, value) in options {
            if trimmed == *pattern {
                return Some(value.clone());
            }
        }
    }
}

pub fn pause_for_enter() {
    let mut input = String::new();
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit.\n").unwrap();
    stdout.flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
}
