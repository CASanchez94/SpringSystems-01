use std::io::{self, Write};
use std::process::Command;


enum FileOperation {
    List(String),           
    Display(String),        
    Create(String, String), 
    Remove(String),         
    Pwd,                    
}

// Perform the selected file operation
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            let status = Command::new("ls")
                .arg(&dir)
                .status()
                .expect("Failed to execute ls");
            if !status.success() {
                eprintln!("Failed to list directory '{}'.", dir);
            }
        }

       FileOperation::Display(file) => {
        let status = Command::new("cat")
        .arg(&file)
        .status()
        .expect("failed to execute cat");
        
        if !status.success(){
                eprintln!("Failed to cat '{}'.", file);
            }
        }

        FileOperation::Create(file, content) => {
            let cmd = format!(" echo '{}' > {} ", content, file);
            let status = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .status()
            .expect("Failed to create file");

            if status.success(){
                println!("File '{}' created successfully", file)
            } else {
                eprintln!("Failed to create file '{}'.", file);
            }
    }


        FileOperation::Remove(file) => {
        let status = Command::new("rm") 
        .arg(&file)
        .status()
        .expect("Failed to execute rm command");

        if status.success() {
        println!("File '{}' removed successfully.", file);
        } else {
        eprintln!("Failed to remove file '{}'.", file);
        }
    }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
            .status()
            .expect("Failed to execute pwd");

            if !status.success(){
                eprintln!("Failed to get a working directory.")
            }
        }
}
}


fn read_input(prompt: &str) -> String {
print!("{}", prompt);
io::stdout().flush().unwrap();
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed input. Try again");
input.trim().to_string()

}


fn main() {
    println!("Welcome to the File Operations Program!");


    loop {

        println!("\nFiles Operation Menu");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");



        let choice = read_input("Enter your Choice (0-5): ");

        match choice.as_str(){
            "1" => {
                let dir = read_input("Enter a file directory: ");
                println!();
                perform_operation(FileOperation::List(dir));
            }
            "2" => {
                let file = read_input("Enter file path: ");
                println!();
                perform_operation(FileOperation::Display(file));
            }
            "3" => {
                let file = read_input("Name of file: ");
                let content = read_input("Enter Content: ");
                println!();
                perform_operation(FileOperation::Create(file, content));
            }
            "4" => {
                let file = read_input("Enter file name: ");
                println!();
                perform_operation(FileOperation::Remove(file));
            }
            "5" => {
                println!();
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("Good Bye!");
                break;
            }
            _=> {
                eprintln!("Invalid Choice. Enter a valid number between 0 - 5: ");

            }
        }
    }
}