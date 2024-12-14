use std::fs::read_to_string;
use std::io::{self};
use std::num::ParseIntError;
use std::fmt;
use std::env::args;
use std::iter::zip;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};


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

#[derive(Clone, Debug)]
struct ParsedInput<T> {
    data: Vec<Vec<T>>,
}

fn is_safe(report: &Vec<AocOutput>) -> bool {
    _is_safe_internal(report).0
}

fn _is_safe_internal(report: &Vec<AocOutput>) -> (bool, usize) {
    let out =
        zip(report.iter(), report.iter().skip(1))
        .fold_while((true, true, true, true, 0), | (safe, range, pos, neg, count), (val1, val2)| {
            let in_range = range && (val2-val1).abs() >= 1 && (val2-val1).abs() <= 3;
            let is_pos = pos && (val2-val1).signum() > 0;
            let is_neg = neg && (val2-val1).signum() < 0;
            match in_range && (is_neg || is_pos) {
                false => Done((false, range, pos, neg, count)),
                true => Continue((safe, in_range, is_pos, is_neg, count+1)),
            }
        }).into_inner();
    (out.0, out.4)
}

fn is_safer(report: &Vec<AocOutput>) -> bool {
    match _is_safe_internal(report) {
        (true,  _) => true,
        (_, count) => {
            let mut new_report = report.clone();
            new_report.remove(count);
            let mut new_report1 = report.clone();
            new_report1.remove(count+1);

            _is_safe_internal(&dbg!(new_report)).0 ||
            _is_safe_internal(&dbg!(new_report1)).0
        }
    }
}

impl ParsedInput<AocOutput> {

    fn solver1(&self) -> Result<usize, AocError> {
        let safe_reports =
            self.data.iter()
                     .filter(|report| is_safe(report))
                     .count();
         Ok(safe_reports) 
    }

    fn solver2(&self) -> Result<usize, AocError> {
        let safer_reports =
            self.data.iter()
                     .filter(|report| is_safer(report))
                     .count();
         Ok(safer_reports) 
    }

}

fn reader(filename: &String) -> Result<String, AocError> {
    Ok(read_to_string(&filename)?)
}

fn parser(input: String) -> Result<ParsedInput<AocOutput>, AocError> {
    let mut data: Vec<Vec<AocOutput>> = Vec::new();
    for line in input.lines() {
        let nums: Vec<AocOutput> = line
                   .split_whitespace()
                   .map(|num| num.parse::<AocOutput>())
                   .collect::<Result<Vec<_>, _>>()?;
        data.push(nums);
    }

    Ok(ParsedInput{data})
}

fn main() {
    let filename = args().nth(1).unwrap_or("input/example_input".to_string());

    let parsed_result = reader(&filename).and_then(parser).unwrap();
    let parsed = parsed_result;
    let solution1 = parsed.solver1().unwrap();
    let solution2 = parsed.solver2().unwrap();
    println!("solution:\npart1: {solution1}\npart2: {solution2}");

}
