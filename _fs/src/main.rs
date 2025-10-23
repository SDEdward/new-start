use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut input = String::new();
    let mut content = String::new();

    println!("Enter file path:");
    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read path");
    let path = Path::new(input.trim());

    /* if !path.exists() {
        println!("does not exist.");
        return;
    } */

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                println!("Failed to create parent directories: {e}");
                return;
            }
        }
    }

    if path.is_dir() {
        println!("the path is a directory, not a file.");
        return;
    }

    println!("Enter content to write:");
    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read content");

    match fs::write(path, content) {
        Ok(_) => match path.canonicalize() {
            Ok(canonical) => println!("Successfully wrote to {}", canonical.display()),
            Err(_) => println!("Successfully wrote to file, but couldn't resolve full path."),
        },
        Err(e) => println!("Failed to write to file: {e}"),
    }
}
