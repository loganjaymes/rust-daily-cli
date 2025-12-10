use std::fs::File;
use std::io;
use chrono::{Local};

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

fn try_open(fname: String, td: String) -> bool {
     if fname.trim() == "today" {
        let mut folder_path = String::from("days/");
        folder_path.push_str(&td);
        println!("{folder_path}");

        let file_open = File::open(&folder_path).unwrap_or_else(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                File::create(&folder_path).unwrap_or_else(|error| {
                    panic!("Problem creating file: {error:?}");
                })
            } else {
                panic!("Problem opening file: {error:?}");
            }
        });

        return true
    }
     return false
}

fn main() {
    let today = start_up();
    println!("What file would you like to open?");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");
    
    try_open(file_name, today);

    /*
     * actually open file should be some shit like
     * task1 | task2 | task3 | task4...
     * true  | false | true  | true....
     */
}
