use std::collections::HashMap;

// #[derive(serde::Deserialize, Debug)]
#[derive(Clone)]
pub struct LGDay {
    pub date: String,
    pub checklist: HashMap<String, String>,
}

impl LGDay {
    pub fn build(d: String, mut cl: HashMap<String, String>) -> LGDay { // might need to put as Result<>
        let date = d;
        let checklist = cl;

        LGDay { date, checklist }
    }
}

/*
impl std::fmt::Display for LGDay {
    fn fmt(&std::fmt::self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Date: {}", date);
        println!("Tasks: ")
    }
}
*/

pub fn edit_date(date: String) -> LGDay {

    // search for day to edit by string)
    // ask what task to edit
    // mark as in/complete
    // return lgday thats edited... i think. that or have it be reference
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_write() {
        unimplemented!();
    }
}

