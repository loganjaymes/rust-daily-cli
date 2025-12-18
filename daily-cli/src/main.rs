use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use chrono::{Local};
use csv::{ReaderBuilder, StringRecord};
use dailycli::{LGDay, edit_date};

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

fn init_file(today: String) -> String { // FIXME change to Result<String, &'static str>
    println!("Locating files...");

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
            .open(&new_file).expect("Failed to create new file.");

            println!("What tasks would you like to add? (FORMAT: task1,task2,task3,...,taskn)");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let clean_input = input.trim().to_string();
            let formatted_input = format!("date,{}", clean_input);
            file.write_all(&formatted_input.as_bytes()).expect("Failed to write to new file.");
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

            return new_file;
            
        },
        _ => {
            // FIXME might be able to just get rid of all this since were using a dedicated reader
            // & we assume the file exists.
            // idk yet
            let mut opened_file = format!("./days/{}.lg", input.trim());
            let mut file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(input).expect("Failed to open file.");

            return opened_file.clone();
            
        }
    };

    // return String::from("Something went wrong :sob:");
}

fn parse_csv(path: String) -> Vec<LGDay> { // essentially whole csv
    let mut builder = ReaderBuilder::new();
    builder.double_quote(false);
    let res = builder.from_path(path);
    
    if res.is_err() {
        println!("HELP ME HELP ME HELP ME!!!");
        std::process::exit(9);
    }
    
    let mut reader = res.unwrap();
    // read header since amt can change
    let headers = reader.headers().unwrap();
    // println!("{:?}", headers);
    let mut header_vals: Vec<String> = Vec::new();
    
    for h in headers.into_iter().skip(1) {
        header_vals.push(String::from(h));
    }
    // TODO split into parse_csv and create_lgday?

    let stored_days: Vec<LGDay> = Vec::new();
    for record in reader.records() {
        let uw_record = record.unwrap();
        let record_vals: Vec<String> = uw_record.iter().skip(1).map(|s| String::from(s)).collect();
        // above converts StringRecord to String
        
        let mut tasks: HashMap<String, String> = header_vals.clone().into_iter().zip(record_vals.into_iter()).collect(); // each day has own checklist

        println!("TASKMAP IS : {:?}", tasks);

        // let str_to_bool = matches!(condition, "true"); // use htis for after you fucjdskjfds
        // ngkjfngkjndskjn DIE

        // TODO:
        // put into hmap, send date and hmap to struct
        // -> store struct in stored_days by date (["20251214", "20251215", ...]).. or something
        // -> have user edit checklist (hmap) based on date
        // -> ie. load file -> "wat day woul you like to edit? (YYYYMMDD)"
        // -> search for day with that date
        // -> list tasks, ask which one to change (so T->F or F->T (literally just set it to not if
        // thats possible in rust))
        // cont til quit
        
        // let day = LGDay;
        // stored_days.push(day) after each creation of lgday
    }
    stored_days
}

fn run(days: Vec<LGDay>) {
    // acutal logic goes here
    // have loop cont taking user input
    // if input !"quit" => call edit_date
    /*
    println!("What date would you like to edit?");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    */

    // edit date => edit task -> edit another task || edit another date
}

fn main() {
    let today = start_up();
    let file_path = init_file(today);
    // println!("{file_path}");
    let days = parse_csv(file_path);
}
