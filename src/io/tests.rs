use midir::MidiInput;

fn print_tests() {
    println!("Available options:");
    println!("none");
}

pub fn select_test(midi: MidiInput) -> Option<usize> {
    print_tests();

    return None;
}
