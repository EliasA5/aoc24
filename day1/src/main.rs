use std::fs::read_to_string;
use std::io::{self};
use std::num::ParseIntError;
use std::fmt;
use itertools::{fold, zip_eq};
use std::env::args;

#[derive(Debug)]
enum AocError {
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
        }
    }
}

impl From<io::Error> for AocError {
    fn from(err: io::Error) -> AocError {
        AocError::Io(err)
    }
}

#[derive(Clone, Debug)]
struct ParsedInput<T> {
    left: Vec<T>,
    right: Vec<T>,
}

impl ParsedInput<i64> {
    fn solver(&mut self) -> Result<i64, AocError> {
        self.left.sort();
        self.right.sort();
        if self.left.len() != self.right.len() {
            return Err(AocError::Solver("unexpected sizes".to_string()));
        };
        let mut distance = 0;
        for (l, r) in zip_eq(&self.left, &self.right){
            let dist = (l - r).abs();
            distance += dist;
        }
        return Ok(distance);
    }

    fn solver2(&self) -> Result<i64, AocError> {
        let mut similarity = 0;
        for l in self.left.iter() {
            let count = fold(self.right.iter(), 0, |acc, &elem| {
                match elem {
                    _ if elem == *l => acc + 1,
                    _ => acc
                }
            });
            similarity += l * count;
        };
        return Ok(similarity);
    }
}

fn reader(filename: &String) -> Result<String, AocError> {
    Ok(read_to_string(&filename)?)
}

fn parser(input: String) -> Result<ParsedInput<i64>, AocError> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let nums: Result<Vec<i64>, ParseIntError> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>())
            .collect();  
        match nums {
            Err(_) => return Err(AocError::Parser("bad input param".to_string())),
            Ok(v) => match v.as_slice(){
                [l, r] => {
                    left.push(*l);
                    right.push(*r);
                }
                _ => return Err(AocError::Parser("too many numbers in line".to_string())),
            }

        } 
    }

    let output = ParsedInput::<i64> {
        left,
        right
    };

    return Ok(output);
}

fn main() {
    let ar: Vec<String> = args().collect();
    let filename = ar.get(1).expect("incorrect number of arguments");

    let parsed_result = reader(filename).and_then(parser).unwrap();
    let mut parsed1 = parsed_result;
    let parsed2 = parsed1.clone();
    let solution1 = parsed1.solver().unwrap();
    let solution2 = parsed2.solver2().unwrap();
    println!("solution:\npart1: {solution1}\npart2: {solution2}");
}
