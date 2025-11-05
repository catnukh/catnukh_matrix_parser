use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "matrix.pest"]

pub struct MatrixParser;