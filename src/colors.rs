// Terminal color output
// Corresponds to bash color codes and print_status function

use colored::Colorize;

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

// Function: print_status
// Purpose: Print colored status messages to console
// Args: color - color code, message - message text
// Modifies: None (outputs to stdout)
// Returns: Prints colored message
pub fn print_status(color: Color, message: &str) {
    let colored_msg = match color {
        Color::Red => message.red(),
        Color::Yellow => message.yellow(),
        Color::Green => message.green(),
        Color::Blue => message.blue(),
    };
    println!("{colored_msg}");
}
