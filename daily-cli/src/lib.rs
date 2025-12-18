use std::collections::HashMap;

// #[derive(serde::Deserialize, Debug)]
pub struct LGDay {
    date: String,
    checklist: HashMap<String, String>,
}

impl LGDay {
    pub fn build(d: &String, cl: HashMap<String, String>) -> LGDay { // might need to put as Result<>
        let date = d.to_string();
        let checklist = cl;

        LGDay { date, checklist }
    }
}

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

