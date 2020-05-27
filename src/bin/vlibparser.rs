use rustick::*;


use std::io::prelude::*;
use std::io;

use std::env;
use std::fs::File;
use crate::ast::*;
use crate::parser::identifier;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let (_, tail) = args.split_at(1);
    for arg in tail {
        println!("parsing {}", arg);
        let s = read_file(arg.clone())?;
        let r = identifier(s.as_bytes());
        println!("{:?}", r);
    }

    Ok(())
}