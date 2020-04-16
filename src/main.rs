// use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

#[macro_use] extern crate lalrpop_util;

mod ast;

mod request;
use request::Request;

lalrpop_mod!(pub parse); // synthesized by LALRPOP

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments");
    }

    let mut var_map: HashMap<String, i32> = HashMap::new();

    if let Ok(lines) = read_lines (args[1].to_string ()) {
        for line in lines {
            if let Ok(line) = line {
                parse_line (line, &mut var_map);
            }
        }
    }
}

fn parse_line(line: String, var_map: &mut HashMap<String, i32>) {
    match parse::StatementParser::new().parse(&line[..]) {
        Ok(_res) => match _res {
            Request::VARIABLE(var) => match var.data {
                Some(value) => {
                    let i32_value = value.evaluate (var_map);
                    (*var_map).insert (var.identifier, i32_value);
                },
                None => panic!("Recieved variable creation request without value")
            }
            Request::PRINT(data) => println!("{}", (*data).evaluate (var_map))
        },
        Err(_res) => panic!("Execution failed: {}", _res)
    };
}
