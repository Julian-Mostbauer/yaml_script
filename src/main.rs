use core::fmt;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Serialize, Deserialize, Debug)]
struct Function {
    name: String,
    content: String,
    return_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Variable {
    name: String,
    content: String,
    content_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Code {
    entry_point: String,
    variables: Vec<Variable>,
    functions: Vec<Function>,
}

impl fmt::Display for Code {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}\n{:#?}", self.variables, self.functions)
    }
}

fn main() {
    let file_path = r"C:\Users\julia\OneDrive\Dokumente\GitHub\yaml_script\src\script.yaml";
    let f = std::fs::File::open(file_path).expect("Could not open file.");
    let code: Code = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{}", code.entry_point);

    println!("{}", code);
}
