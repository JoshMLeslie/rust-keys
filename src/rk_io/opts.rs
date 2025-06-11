use midir::MidiInputConnection;

fn print_opts() {
    println!("Available options:");
    println!("none");
}

pub fn select_opt() -> Option<MidiInputConnection<()>> {
    print_opts();
    return None;
}
