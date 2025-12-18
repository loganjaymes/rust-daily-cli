// REMOVE THIS LATER
#![allow(unused)]
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use chrono::{Local};
use csv::{Writer, ReaderBuilder, StringRecord};
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

fn init_file(today: &String) -> String { // FIXME change to Result<String, &'static str>
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
                    file.write_all(b"incomplete,");
                } else {
                    file.write_all(b"incomplete");
                }
            }

            return new_file;
            
        },
        _ => {
            // although using reader crate, if the file is empty/DNE we need to initialize it
            let mut opened_file = format!("./days/{}.lg", input.trim());
            let mut file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(input).expect("Failed to open file.");

            return opened_file.clone();
            
        }
    };
}

fn parse_csv(path: &String, today: &String) -> Vec<LGDay> { // essentially whole csv as a vector w/ DS
    let mut builder = ReaderBuilder::new();
    builder.double_quote(false);
    let res = builder.from_path(path);
    
    if res.is_err() {
        println!("HELP ME HELP ME HELP ME!!!");
        std::process::exit(9);
    }
    
    let mut reader = res.unwrap();
    // read header since amt of tasks can change depending on file
    let headers = reader.headers().unwrap();
    let mut header_vals: Vec<String> = Vec::new();
    
    for h in headers.iter().skip(1) {
        header_vals.push(String::from(h));
    }

    let mut stored_days: Vec<LGDay> = Vec::new();
    let mut today_found = false; // TODO this is a naive approach im p sure
    for record in reader.records() {
        let uw_record = record.unwrap();
        let record_vals: Vec<String> = uw_record.iter().map(|s| String::from(s)).collect();
        // above converts StringRecord to String
        let record_date = record_vals[0].clone(); // shouldnt be moving since were skipping 1st index
                                               // in tasks definition, but rust says it's a borrow
        if record_date == *today { // if true here we can skip code outside of scope
            today_found = true;
        }

        let mut tasks: HashMap<String, String> = header_vals.clone().into_iter().zip(record_vals.into_iter().skip(1)).collect(); // each day has own checklist

        let day = LGDay {
            date: record_date,
            checklist: tasks,
        };

        stored_days.push(day); // after each creation of lgday
    }

    if today_found == false {
        let mut record_vals: Vec<String> = Vec::new();
        for h in header_vals.iter() {
            record_vals.push(String::from("incomplete"));
        }

        let mut tasks: HashMap<String, String> = header_vals.clone().into_iter().zip(record_vals.into_iter()).collect(); // each day has own checklist

        let day = LGDay {
            date: today.to_string(),
            checklist: tasks,
        };

        stored_days.push(day); // dont need to update today_found since itll be dropped after func
    }

    stored_days
}

fn run(mut days: Vec<LGDay>, path: String) {
    // TODO have editing a date and editing the tasks separate funcs in lib.rs
    // implementing it this way likely makes looping for input a lot easier.
    let mut stored_day = LGDay { date: String::from(""), checklist: HashMap::new()}; // FIXME make def vals in struct
    
    // check if today is in vector, if not then append
    // probably better way of doing this, ie. lets say user doesnt want to add new entry, then
    // this wouldn't be the best solution, but idk as of now
    
    println!("For file {}, days include:", &path);
    
    for day in &days {
        println!("{}", day.date);
    }

    println!("What date would you like to edit?");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let clean_input = input.trim().to_string();

    for day in &days {
        if day.date == clean_input {
            stored_day = day.clone();
        } 
    }
    
    let scuffed_error = stored_day.date.to_string(); // FIXME figure out a better way to handle
                                                     // this lmfao
    if scuffed_error == "" {
        panic!("Date not found");
    }

    // TODO pretty print struct fields and hashmap especially (ie. linebreaks)
    println!("For day {}, possible tasks are {:?}", &stored_day.date, &stored_day.checklist);

    let mut stored_task = String::new();
    println!("What task would you like to edit?");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let clean_input = input.trim().to_string();

    if stored_day.checklist.contains_key(&clean_input) {
        stored_task = clean_input.clone();
    } else {
        panic!("Could not find task {}", clean_input);
    }

    println!("What value would you like to set it to? (complete, incomplete, in-progress)");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let clean_input = input.trim().to_string();
    
    *stored_day.checklist.get_mut(&stored_task).unwrap() = clean_input;

    println!("Value of checklist after changing: {:?}", &stored_day.checklist);

    // replace in vector
    for day in &mut days {
        if day.date == stored_day.date {
            *day = stored_day.clone();
        }
    }

    let mut new_header: Vec<String> = Vec::new();
    new_header.push(String::from("date"));
    let header_to_vec: Vec<String> = stored_day.checklist.keys().cloned().collect();
    new_header.extend(header_to_vec);
    // dont need to convert from vector ["date", "t1", ..., "tn"] since csv hadnles it
    
    let mut res = Writer::from_path(path);
    if res.is_err() {
        println!("HELP ME HELP ME HELP ME!!!");
        std::process::exit(9);
    }

    let mut writer = res.unwrap();
    writer.write_record(&new_header);
    // rewrite entire 'days' vector from memory since we cannot write to a specific place in file
    for d in &days { 
        println!("{:?}", d.checklist);
        let mut new_record: Vec<String> = Vec::new();
        new_record.push(String::from(d.date.clone()));
        let new_vals: Vec<String> = d.checklist.values().cloned().collect();
        new_record.extend(new_vals);
        writer.write_record(&new_record);
    }

    /*
     * logic:
     * update hmap val for stored_task      DONE
     * iterate over days and rewrite all    HELP
     *
     *                                      write header
     *                                      for day in lgday
     *                                          write day1.date, checklist.vals
     *
     * ???                                  ????
     * profit                               PLEASE
     * done                                 ALSO PLEASE
     */


    // TODO have loop cont taking user input
    // if input !"quit" => call edit_date
    // edit date => edit task -> edit another task || edit another date
}

fn main() {
    let today = start_up();
    let file_path = init_file(&today);
    // println!("{file_path}");
    let days = parse_csv(&file_path, &today);
    // implement view after selecting date
    run(days, file_path);
}
