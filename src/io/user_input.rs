use std::io::{stdin, stdout, Write};

pub fn get_input<T: Clone>(
    prompt: &str,
    options: &[(&str, T)],
) -> Option<T> {
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
