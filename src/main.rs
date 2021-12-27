struct Project {
    name: String
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: String::from(name)
        }
    }
}

struct Felix {
    projects: Vec<Project>,
    nationality: String,
    sussiness: i8
}

impl Felix {
    pub fn new(nationality: &str /* just a string literal*/) -> Felix {
        if nationality != "Italian" {
            println!("Usually felix is Italian, but ok...");
        }

        Felix {
            projects: vec!(),
            nationality: String::from(nationality),
            sussiness: 0
        }
    }
}

fn main() {
    let felix = Felix::new("Sussy");

    println!("{}", felix.nationality)
}
