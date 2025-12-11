use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use chrono::{Local};

// TODO refactor into lib crate cause ts butters

struct Config {
    tasks: Vec<String>,
}

struct DailyFile {
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

fn load_config() -> Config {
    let mut created = false;

    let mut config_open = match fs::File::open(&"config.log") {
        Ok(file) => {
            println!("Config file successfully located...");
            file
        }
        // FIXME will just break if config not present. will write to newly created config file but throws exception after
        // not sure if its a permissions thing or logic error in code
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            println!("Config file undetected, creating default...");
            created = true;
            fs::File::create(&"config.log").unwrap_or_else(|e| { panic!("Problem creating file: {e:?}") } )
        }

        Err(error) => {
            panic!("Problem opening config: {error:?}");
        }
    };

    if created { config_open.write_all(b"Task1,Task2,Task3"); }

    // READ FILE NAOW
    let mut cft = String::new();
    config_open.read_to_string(&mut cft).unwrap();
    let t: Vec<String> = cft.split(',').map(|s| s.to_string()).collect();

    let c = Config {
        tasks: t.clone(),
    };

    println!("!! DEBUG !! {:?}", c.tasks);

    c
}

fn try_open(fname: &String, td: String, c: Config) -> (bool, String) {
    // TODO: make file struct that is just filled from here
    // TODO: MIGHT NEED TO USE OPENOPTIONS LMFAOOOOOOOOOOOO ggbruh
    // stores stuff like pathname and prolly hashmap of task names and vals
    let mut folder_path = String::from("days/");
    let mut created = false;

    if fname.trim() == "today" {
        folder_path.push_str(&td);
        // println!("{folder_path}");
        
        // can probably refactor below somehow without having to copy paste in both ifs
        let mut file_open = match fs::File::open(&folder_path) {
            Ok(file) => {
                println!("File '{}' exists, opening...", folder_path);
                return (true, folder_path)
            }

            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                println!("File {}' not found, creating...", folder_path);
                created = true;
                fs::File::create(&folder_path).unwrap_or_else(|e| { panic!("Problem creating file: {e:?}") } )
            }

            Err(error) => {
                panic!("Problem opening file: {error:?}");
            }
        };
        
        if created {
            println!("Writing boilerplate..."); // FIXME bp based on struct
            file_open.write_all(c.tasks.join(",").as_bytes()).expect("Cannot write to file");
            file_open.write_all(b"\n");
            let mut iter = c.tasks.iter().peekable();

            while let Some(element) = iter.next() {
                if !iter.peek().is_none() {
                    file_open.write_all(b"false,");
                } else {
                    file_open.write_all(b"false");
                }
            }
        }

        // (true, folder_path)

    } else { // TODO: validate string to ensure its in YYYYMMDD format. ignore for now 
        folder_path.push_str(&fname.trim());
        // println!("{folder_path}");

        let mut file_open = match fs::File::open(&folder_path) {
            Ok(file) => {
                println!("File '{}' exists, opening...", folder_path);
                file
            }

            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                println!("File {}' not found, creating...", folder_path);
                created = true;
                fs::File::create(&folder_path).unwrap_or_else(|e| { panic!("Problem creating file: {e:?}") } )
            }

            Err(error) => {
                panic!("Problem opening file: {error:?}");
            }
        };

        if created {
            println!("Writing boilerplate..."); // FIXME bp based on struct
            file_open.write_all(c.tasks.join(",").as_bytes()).expect("Cannot write to file");
            file_open.write_all(b"\n");
            let mut iter = c.tasks.iter().peekable();

            while let Some(element) = iter.next() {
                if !iter.peek().is_none() {
                    file_open.write_all(b"false,");
                } else {
                    file_open.write_all(b"false");
                }
            }
        }

        // (true, folder_path)
    }

     // return (false, String::from(""))
     // ^^ shouldnt be necessary bc error handling but anyway...s ^^
    
    let mut df = DailyFile {
        file_path: folder_path.clone(),
        checklist: HashMap::from(
            [
                (String::from("LeetCode"), false), 
                (String::from("Practice"), false), 
                (String::from("Program"), false), 
                (String::from("Read"), false),
            ]
        ), // FIXME: will need to be refactored for customization (so not hardcoded tasks). probably through a config.txt. But that's for later. lamo
    };

    for (task, val) in &df.checklist {
        println!("!! DEBUG !! {task} : {val}");
    }
    
    return (true, folder_path)
}

fn main() {
    let today = start_up();
    let conf = load_config();
    println!("What file would you like to open?");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");
    let pair = try_open(&file_name, today, conf);

    if pair.0 {
        // println!("File exists");
        let contents = fs::read_to_string(&pair.1).expect("Should read"); // FIXME read from struct
        println!("{}", contents);
    }

    /*
     * actually open file should be some shit like
     * task1 | task2 | task3 | task4...
     * true  | false | true  | true....
     */
}
