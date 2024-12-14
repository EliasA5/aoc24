use std::fs::read_to_string;
use std::io::{self};
use std::num::ParseIntError;
use std::fmt;
use std::env::args;


#[derive(Debug)]
enum AocError {
    NotImplemented,
    Io(io::Error),
    Parser(String),
    Solver(String),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::Io(ref err) => write!(f, "IO error: {}", err),
            AocError::Parser(ref err) => write!(f, "Parser error: {}", err),
            AocError::Solver(ref err) => write!(f, "Solver error: {}", err),
            AocError::NotImplemented => write!(f, "Not Implemented"),
        }
    }
}

impl From<io::Error> for AocError {
    fn from(err: io::Error) -> AocError {
        AocError::Io(err)
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> AocError {
        AocError::Parser(err.to_string())
    }
}

type AocOutput = i64;
type AocInput = i64;

#[derive(Clone, Debug)]
struct ParsedInput<T> {
    data: T,
}

impl ParsedInput<AocInput> {

    fn solver1(&self) -> Result<AocOutput, AocError> {
        Err(AocError::NotImplemented)
    }

    fn solver2(&self) -> Result<AocOutput, AocError> {
        Err(AocError::NotImplemented)
    }

}

fn reader(filename: &String) -> Result<String, AocError> {
    Ok(read_to_string(&filename)?)
}

fn parser(input: String) -> Result<ParsedInput<AocInput>, AocError> {
    Err(AocError::NotImplemented)
}

fn main() {
    let ar: Vec<String> = args().collect();
    let filename = ar.get(1).expect("incorrect number of arguments");

    let parsed_result = reader(filename).and_then(parser).unwrap();
    let parsed1 = parsed_result;
    let parsed2 = parsed1.clone();
    let solution1 = parsed1.solver1().unwrap();
    let solution2 = parsed2.solver2().unwrap();
    println!("solution:\npart1: {solution1}\npart2: {solution2}");
}
