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
// NOTE: Adds emoji prefix for HIGH RISK (🚨) and MEDIUM RISK (⚠️) to match bash output
pub fn print_status(color: Color, message: &str) {
    // Add emoji prefix to match bash output format
    let formatted_msg = if message.starts_with("HIGH RISK:") {
        format!("🚨 {message}")
    } else if message.starts_with("MEDIUM RISK:") {
        format!("⚠️  {message}")
    } else {
        message.to_string()
    };

    let colored_msg = match color {
        Color::Red => formatted_msg.red(),
        Color::Yellow => formatted_msg.yellow(),
        Color::Green => formatted_msg.green(),
        Color::Blue => formatted_msg.blue(),
    };
    println!("{colored_msg}");
}
