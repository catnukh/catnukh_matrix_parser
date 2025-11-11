use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use catnukh_matrix_parser::{parse_file, Command, Matrix};

#[derive(Parser, Debug)]
#[command(author="catnukh", version, about="Parser for matrix operations")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(clap::Subcommand, Debug)]
enum CliCommand {
    Parse {file_path: String},
    Credits,
}

fn run_iterpreter(commands: Vec<Command>) -> Result<()> {
    let mut matrix_storage: HashMap<String, Matrix> = HashMap::new();
    for command in commands {
        match command {
            Command::Define(name, matrix) => {
                println!("Defined matrix {} ({}*{})", name, matrix.rows_num, matrix.cols_num);
                matrix_storage.insert(name, matrix);
            }
            Command::Add(name_1, name_2) => {
                println!("Adding matrix {} and matrix {}", name_1, name_2);
                let matrix_1 = matrix_storage.get(&name_1);
                let matrix_2 = matrix_storage.get(&name_2);
                match (matrix_1, matrix_2) {
                    (Some(mat_1), Some(mat_2)) => {
                        if mat_1.rows_num != mat_2.rows_num || mat_1.cols_num != mat_2.cols_num {
                            eprintln!("Error. Dimentions of matrix aren't equal");
                            eprintln!("{}: {}*{}", name_1, mat_1.rows_num, mat_1.cols_num);
                            eprintln!("{}: {}*{}", name_2, mat_2.rows_num, mat_2.cols_num);
                        }
                        else {
                            let mut result = Vec::new();
                            for row in 0..mat_1.rows_num {
                                let mut res_row = Vec::new();
                                for col in 0..mat_1.cols_num {
                                    let sum = mat_1.matrix[row][col] + mat_2.matrix[row][col];
                                    res_row.push(sum);
                                }
                                result.push(res_row);
                            }
                            println!("Addition result: {:?}", result);
                        }
                    },
                    (None, _) => {eprintln!("Error. Matrix {} are not defined", name_1);},
                    (_, None) => {eprintln!("Error. Matrix {} are not defined", name_2);}
                }
            }
            Command::Subtract(name_1, name_2) => {
                println!("Subtracting matrix {} and matrix {}", name_1, name_2);
                let matrix_1 = matrix_storage.get(&name_1);
                let matrix_2 = matrix_storage.get(&name_2);
                match (matrix_1, matrix_2) {
                    (Some(mat_1), Some(mat_2)) => {
                        if mat_1.rows_num != mat_2.rows_num || mat_1.cols_num != mat_2.cols_num {
                            eprintln!("Error. Dimentions of matrix aren't equal");
                            eprintln!("{}: {}*{}", name_1, mat_1.rows_num, mat_1.cols_num);
                            eprintln!("{}: {}*{}", name_2, mat_2.rows_num, mat_2.cols_num);
                        }
                        else {
                            let mut result = Vec::new();
                            for row in 0..mat_1.rows_num {
                                let mut res_row = Vec::new();
                                for col in 0..mat_1.cols_num {
                                    let sub = mat_1.matrix[row][col] - mat_2.matrix[row][col];
                                    res_row.push(sub);
                                }
                                result.push(res_row);
                            }
                            println!("Subtraction result: {:?}", result);
                        }
                    },
                    (None, _) => {eprintln!("Error. Matrix {} are not defined", name_1);},
                    (_, None) => {eprintln!("Error. Matrix {} are not defined", name_2);}
                }
            }
            Command::Multiply(name_1, name_2) => {
                println!("Multiplying matrix {} and matrix {}", name_1, name_2);
                let matrix_1 = matrix_storage.get(&name_1);
                let matrix_2 = matrix_storage.get(&name_2);
                match (matrix_1, matrix_2) {
                    (Some(mat_1), Some(mat_2)) => {
                        if mat_1.cols_num != mat_2.rows_num {
                            eprintln!("Error. The number of columns in first matrix must be equal to the number of rows in second");
                            eprintln!("{}: {}*{}", name_1, mat_1.rows_num, mat_1.cols_num);
                            eprintln!("{}: {}*{}", name_2, mat_2.rows_num, mat_2.cols_num);
                        }
                        else {
                            let mut result = Vec::new();
                            for i in 0..mat_1.rows_num {
                                let mut res_row = Vec::new();
                                for j in 0..mat_2.cols_num {
                                    let mut product = 0.0;
                                    for k in 0..mat_1.cols_num {
                                        product += mat_1.matrix[i][k]*mat_2.matrix[k][j];
                                    }
                                    res_row.push(product);
                                }
                                result.push(res_row);
                            }
                            println!("Multiplication result: {:?}", result);
                        }
                    },
                    (None, _) => {eprintln!("Error. Matrix {} are not defined", name_1);},
                    (_, None) => {eprintln!("Error. Matrix {} are not defined", name_2);}
                }
            }
            Command::Scale(name, number) => {
                let matrix = matrix_storage.get(&name);
                match matrix  {
                    Some(mat) => {
                        let mut res = Vec::new();
                        for i in 0..mat.rows_num {
                            let mut res_row = Vec::new();
                            for j in 0..mat.cols_num {
                                let product = mat.matrix[i][j] * number;
                                res_row.push(product);
                            }
                            res.push(res_row);
                        }
                        println!("Scaling result {:?}", res);
                    },
                    None => {eprintln!("Error. Matrix not found");},
                }
            }
        }
    }
    println!("---End of program---");
    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        CliCommand::Parse{ file_path } => {
            let content = fs::read_to_string(&file_path).with_context(|| format!("Can't read file {}", file_path))?;
            match parse_file(&content) {
                Ok(commands) => {run_iterpreter(commands)?;},
                Err(e) => {eprintln!("Parsing error {}", e);}
            }
        },

        CliCommand::Credits => {
            println!("Developed by catnukh");
            println!("GitHub: https://github.com/catnukh/catnukh_matrix_parser");
        }
    }

    Ok(())
}
