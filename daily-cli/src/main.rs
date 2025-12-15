use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use chrono::{Local};

struct LGFile {
    file_path: String,
    checklist: HashMap<String, bool>,
}


fn start_up() -> String {
    let uname = users_native::get_current_username();
    println!("Good afternoon, {}", uname);
    
    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d");
    let formatted_time = now.format("%H:%M:%S");
    let today = now.format("%Y%m%d");
    println!("It is currently {} at {}", formatted_date, formatted_time);

    today.to_string()
}

fn load_file(today: String) /* -> Result<Config, io::Error > */{
    println!("Locating file...");

    let paths = fs::read_dir("./days").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(OsStr::to_str) == Some("lg") {
            println!("{}", path.display());
        }
    }
    println!("Would you like to use one of the above files or create a new one? (if no files were listed, type 'new', else, type just the name of the file with no extension).");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let new = String::from("new");
    match input.trim() {
        "new" => {
            println!("What would you like to name the file? DO NOT ADD ANY EXTENSIONS");
            let mut input = String::new();
            io::stdin().read_line(&mut input);

            let mut new_file = format!("./days/{}.lg", input.trim());
            let mut file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(new_file).expect("Failed to create new file.");

            println!("What tasks would you like to add? (FORMAT: task1,task2,task3,...,taskn)");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let clean_input = input.trim().to_string();
            let formatted_input = format!("date,{}", clean_input);
            file.write_all(formatted_input.clone().as_bytes()).expect("Failed to write to new file.");
            let formatted_today = format!("\n{},", today);
            file.write_all(formatted_today.as_bytes());

            // count num commas => repeat that many times + 1 for false,false,false,false
            let mut tasks = clean_input.split(",").peekable();
            while let Some(t) = tasks.next() {
                if !tasks.peek().is_none() {
                    file.write_all(b"false,");
                } else {
                    file.write_all(b"false");
                }
            }
            
        },
        _ => {
            let mut opened_file = format!("./days/{}.lg", input.trim());
            let mut file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(input).expect("Failed to open file.");
            
            // test: actually read file make sure it works and shit
            let content = fs::read_to_string(opened_file).expect("File read unsucc.");
            println!("{content}");

            /* LGFile { // only need most recent (so last) line

            }*/
        }
    };
}

fn main() {
    let today = start_up();
    load_file(today);
}
