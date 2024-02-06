use std::io::stdin;

// Generates a text menu for the user to select between choices by
// inputting a number from 0 to choices.len()-1
pub fn menu_input(prompt: String, choices: Vec<String>) -> usize {
    // Display the choices to the user
    println!("{}", prompt);
    for i in 0..choices.len() {
        println!("{}: {}", i, choices[i]);
    }

    loop {
        // Gather input repeatedly, til the user enters a valid number
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error when reading from stdin");
        match input.trim().parse::<i32>() {
            Ok(n) => {
                if 0 <= n && n < choices.len() as i32 {
                    return n as usize;
                } else {
                    println!("\nMake sure your input is one of the choices.");
                }
            },
            Err(e) => {
                println!("\nMake sure you are entering a valid number.\nError: {}", e);
            }
        }
    }
}