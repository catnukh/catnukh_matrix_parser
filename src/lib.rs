use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pair;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "src/matrix.pest"]

pub struct MatrixParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error("Mismatched row length, expected {expected:?}, found {found:?}")]
    MismatchedRowLength{expected: usize, found: usize},

    #[error("Error parsing number {0}")]
    ParseInvalidNumber(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub matrix: Vec<Vec<f64>>,
    pub rows_num: usize,
    pub cols_num: usize,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Define(String, Matrix),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Scale(String, f64)
}

fn parse_matrix(matrix_pair: Pair<Rule>) -> Result<Matrix, ParseError> {
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    let mut cols: Option<usize> = None;

    for row in matrix_pair.into_inner() {
        let mut current_row: Vec<f64> = Vec::new();
        for number_pair in row.into_inner() {
            let number_str = number_pair.as_str();
            let number = number_str
                .parse::<f64>()
                .map_err(|_| ParseError::ParseInvalidNumber(number_str.to_string()))?;
            current_row.push(number);
        }

        let current_cols = current_row.len();
        if let Some(expected) = cols {
            if current_cols != expected {
                return Err(ParseError::MismatchedRowLength {
                    expected,
                    found: current_cols,
                });
            }
        } else {
            cols = Some(current_cols);
        }

        matrix.push(current_row);
    }

    let rows_num = matrix.len();
    let cols_num = cols.unwrap_or(0);

    Ok(Matrix {
        matrix,
        rows_num,
        cols_num,
    })
}

fn parse_operation(pair: Pair<Rule>) -> Result<Command, ParseError> {
    match pair.as_rule() {
        Rule::mat_def => {
            let mut inner_pair = pair.into_inner();
            let name = inner_pair.next().unwrap().as_str().trim().to_string();
            let mat = inner_pair.next().unwrap();
            let matrix = parse_matrix(mat)?;
            Ok(Command::Define(name, matrix))
        },
        Rule::add => {
            let mut inner_pair = pair.into_inner();
            let name_1 = inner_pair.next().unwrap().as_str().trim().to_string();
            let name_2 = inner_pair.next().unwrap().as_str().trim().to_string();

            Ok(Command::Add(name_1, name_2))
        },
        Rule::subtract => {
            let mut inner_pair = pair.into_inner();
            let name_1 = inner_pair.next().unwrap().as_str().trim().to_string();
            let name_2 = inner_pair.next().unwrap().as_str().trim().to_string();
            Ok(Command::Subtract(name_1, name_2))
        },
        Rule::mult => {
            let mut inner_pair = pair.into_inner();
            let name_1 = inner_pair.next().unwrap().as_str().trim().to_string();
            let name_2 = inner_pair.next().unwrap().as_str().trim().to_string();
            Ok(Command::Multiply(name_1, name_2))
        },
        Rule::scale => {
            let mut inner_pair = pair.into_inner();
            let name_1 = inner_pair.next().unwrap().as_str().trim().to_string();
            let num_str = inner_pair.next().unwrap().as_str();
            let number = num_str.parse::<f64>().map_err(|_| ParseError::ParseInvalidNumber(num_str.to_string()))?;
            Ok(Command::Scale(name_1, number))
        },
        _ => unreachable!("Wrong operation"),
    }
}

pub fn parse_file(matrix_file: &str) -> Result<Vec<Command>, ParseError> {
    let pairs = MatrixParser::parse(Rule::file, matrix_file)?.next().unwrap();
    let mut commands = Vec::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::operation => {
                let inner_pair = pair.into_inner().next().unwrap();
                let command = parse_operation(inner_pair)?;
                commands.push(command);
            }
            Rule::EOI => {break;},
            _ => {},
        }
    }

    Ok(commands)
}