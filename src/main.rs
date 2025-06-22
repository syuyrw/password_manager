use std::io;
use std::fs;
use std::collections::HashMap;
use serde_json::Value;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Password
    let my_password = "noneofyourbusiness";

    println!("\nEnter password to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input != my_password {
        println!("Wrong. STAY AWAY FROM MY PASSWORDS!!!");
        return Ok(());
    }

    // Check if user wants to write new password
    println!("\nWould you like to create a new password?");
    let mut new_answer = String::new();
    io::stdin().read_line(&mut new_answer).expect("Failed to read line");

    let response = new_answer.trim().to_lowercase();

    if response == "yes" || response == "y" {
        println!("\nEnter the URL for website this password is for:");

    // Get website URL from user
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input).expect("Failed to read line");
    let url_input = url_input.trim().to_string(); // Trim input

    // Get username from user
    println!("\nEnter the username for the website this password is for:");

    let mut username_input = String::new();
    io::stdin().read_line(&mut username_input).expect("Unable to read line");
    let username_input = username_input.trim().to_string(); // Trim input

    // Get the password from user
    println!("\nEnter the password for the website this is for:");

    let mut password_input = String::new();
    io::stdin().read_line(&mut password_input).expect("Unable to read line");
    let password_input = password_input.trim().to_string(); // Trim input

    // Read json file
    let file_path = "passwords.json";
    let file_contents = fs::read_to_string(file_path).unwrap_or("{}".to_string());

    // Hashmap that contains all usernames and passwords
    let mut all_passwords: HashMap<String, HashMap<String, String>> = serde_json::from_str(&file_contents).unwrap_or_else(|_| HashMap::new());

    // Create new entry
    let mut password_data = HashMap::new();
    password_data.insert("username".to_string(), username_input);
    password_data.insert("password".to_string(), password_input);

    all_passwords.insert(url_input, password_data);

    // Write to json file
    let json_string = serde_json::to_string_pretty(&all_passwords)?;
    fs::write(file_path, json_string)?;

    println!("\nPassword saved successfully!");
    Ok(())
    } else if response == "no" || response == "n" {
        // Check if user wants to access a password
        println!("\nWould you like to find a password?");
        let mut new_answer = String::new();
        io::stdin().read_line(&mut new_answer).expect("Failed to read line");

        let response = new_answer.trim().to_lowercase();

        if response == "yes" || response == "y" {
            // Read file into string
            let json_string = fs::read_to_string("passwords.json")?;

            // Parse file
            let data: HashMap<String, Value> = serde_json::from_str(&json_string)?;

            // Get user input for which url to look up
            println!("\nType the URL for the website you want to find the password for:");
            let mut new_answer = String::new();
            io::stdin().read_line(&mut new_answer).expect("Failed to read line");

            // Find password from URL
            let lookup_key = new_answer.trim().to_string();
            if let Some(user_info) = data.get(&lookup_key) {
                println!("\nFound user '{}': {}", lookup_key, user_info);
            } else {
                println!("URL '{}' not found.", lookup_key);
            }
            Ok(())
        } else {
        println!("\nHave a good day");
        return Ok(());
        } 
    } else {
        println!("\nHave a good day");
        Ok(())
    }  

}
