use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut errors = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            errors.push(line.to_string());
        }
    }

    errors
}

fn main() -> Result<(), Error> {
    /*
    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            let error_logs = extract_errors(content.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => {
                    println!("Successfully wrote errors text to file");
                }
                Err(err) => {
                    println!("Error, failed to write errors text to file: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error, failed to read file: {}", err);
        }
    }
    */

    // Alternative using Result expect
    // let text = fs::read_to_string("logs.txt").expect("Failed to read file");

    // let error_logs = extract_errors(text.as_str());

    // fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write errors text to file")

    // Handle errors with Result ? (try operator) and early return
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());

    fs::write("errors.txt", error_logs.join("\n"))?;

    print!("{}", error_logs.join("\n"));

    Ok(())
}
