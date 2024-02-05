use std::io::stdin; // for user input

// Generates a text menu for the user to select between choices by
// inputting a number from 0 to choices.len()-1
fn menu_input(prompt: String, choices: Vec<String>) -> usize {
    // Display the choices to the user
    println!("{}", prompt);
    for i in 0..choices.len() {
        println!("{}: {}", i, choices[i]);
    }

    loop {
        // Gather input repeatedly, til the user enters a valid number
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(n) => {
                if 0 <= n && n < choices.len().try_into().unwrap() {
                    return n.try_into().unwrap();
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

fn main() {
    let choice = menu_input("Select Type:".to_string(), 
        vec!["Server".to_string(), "Client".to_string()]
    );
    println!("You chose {}.", choice);
}
