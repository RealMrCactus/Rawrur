use std::io::Write;
use crossterm::style::*;

pub fn yn_prompt_install(prompt: String, noconf: bool) -> bool {
    // prompt the user to install
    print!("\nWould you like to install {}? [", prompt);
    crossterm::style::SetForegroundColor(Color::Green);
    print!("Y");
    crossterm::style::SetForegroundColor(Color::White);
    print!("/");
    crossterm::style::SetForegroundColor(Color::Red);
    print!("n");
    crossterm::style::SetForegroundColor(Color::White);
    print!("] ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    // get their answer
    input = std::io::stdin().read_line(&mut input).unwrap().to_string();

    // check if input is valid.
    if input.trim().to_lowercase() != "y" || input.trim().to_lowercase() != "n" || input.trim().is_empty() && !noconf {
        crossterm::style::SetForegroundColor(Color::Red);

        println!("\n Canceling...");

        crossterm::style::SetForegroundColor(Color::White);

        yn_prompt_install(prompt, noconf);
        return false;  

    } else if input.to_lowercase().trim() == "y" || input.trim().is_empty() && noconf {
        return true;
    } else {
        crossterm::style::SetForegroundColor(Color::Red);
        eprintln!("Error getting users input selection.");
        return false;
    }    
}