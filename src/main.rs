use core::fmt;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::borrow::Borrow;

fn form(s: String) -> String {
    s.replace(")", " ) ").replace("(", " ( ")
}

fn split_code(s: String) -> Vec<String> {
    s.split(&['(', ')'])
        .map(|x| x.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

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
impl Code {
    fn get_entry_function(&self) -> Option<&Function> {
        let fn_name = &self.entry_point;
        self.functions.iter().find(|f| &f.name == fn_name)
    }

    fn compute_func(&self, inp: Vec<String>) -> Vec<String> {
        let fun = inp.first().expect("no function provided").to_owned();
        let args = &inp[1..];

        let mut out: Vec<String> = Vec::new();

        match fun.as_str() {
            "fn_call" => {
                let found_functions = self
                    .functions
                    .iter()
                    .filter(|f| &f.name == args.first().expect("no args provided"))
                    .collect::<Vec<&Function>>();
                assert_eq!(found_functions.len(), 1);
                let found_function = found_functions.first().unwrap();
                let found_code = found_function.content.clone();

                out.extend_from_slice(&self.compute_func(split_code(form(found_code))));
            }
            "println" => {
                args.iter().for_each(|s| {
                    let is_litteral = (s.starts_with('\"') || s.starts_with('\''))
                        && (s.ends_with('\"') || s.ends_with('\''));
                    if is_litteral {
                        println!("{}", s)
                    } else {
                        let found_variables = self
                            .variables
                            .iter()
                            .filter(|&v| v.name.trim() == s.trim())
                            .collect::<Vec<&Variable>>();

                        assert_eq!(found_variables.len(), 1);
                        let found_variable = found_variables.first().unwrap();
                        println!("{}", found_variable.content)
                    }
                });
            }
            _ => (),
        };

        out
    }
}

fn main() {
    let file_path = r"C:\Users\julia\OneDrive\Dokumente\GitHub\yaml_script\src\script.yaml";
    let f = std::fs::File::open(file_path).expect("Could not open file.");

    let code: Code = serde_yaml::from_reader(f).expect("Could not read values.");

    let entry_fn = code
        .get_entry_function()
        .expect("Unable to locate entry point");

    let formated_code = form(entry_fn.content.to_owned());

    let split_code = split_code(formated_code);

    let res = code.compute_func(split_code);

    println!("{:?}", res);
    // println!("{}", code);
}
