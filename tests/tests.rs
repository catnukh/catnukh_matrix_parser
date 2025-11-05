use anyhow::Result;
use catnukh_matrix_parser::{MatrixParser, Rule};
use pest::Parser;

#[test]
fn test_number() -> Result<()> {
    let num1 = MatrixParser::parse(Rule::number, "123.456")?.next().unwrap();
    assert_eq!(num1.as_str(), "123.456");

    let num2 = MatrixParser::parse(Rule::number, "-123")?.next().unwrap();
    assert_eq!(num2.as_str(), "-123");

    let num3 = MatrixParser::parse(Rule::number, "-123.456")?.next().unwrap();
    assert_eq!(num3.as_str(), "-123.456");

    let not_num1 = MatrixParser::parse(Rule::number, "abc");
    assert_eq!(not_num1.is_err(), true);

    let partial1 = MatrixParser::parse(Rule::number, "123.34.56");
    assert_eq!(partial1.is_err(), true);

    let partial2 = MatrixParser::parse(Rule::number, "1hello");
    assert_eq!(partial2.is_err(), true);

    Ok(())
}

#[test]
fn test_name() -> Result<()> {
    let a = MatrixParser::parse(Rule::name_of_matrix, "A")?.next().unwrap();
    assert_eq!(a.as_str(), "A");

    let my_matrix_1 = MatrixParser::parse(Rule::name_of_matrix, "my_matrix_1")?.next().unwrap();
    assert_eq!(my_matrix_1.as_str(), "my_matrix_1");

    let empty = MatrixParser::parse(Rule::name_of_matrix, "");
    assert_eq!(empty.is_err(), true);

    let num = MatrixParser::parse(Rule::name_of_matrix, "23");
    assert_eq!(num.is_err(), true);

    Ok(())
}

#[test]
fn test_row() -> Result<()> {
    let row1 = MatrixParser::parse(Rule::row, "[1, 2, 3]")?.next().unwrap();
    assert_eq!(row1.as_str(), "[1, 2, 3]");

    let row2 = MatrixParser::parse(Rule::row, "[123.456, -78]")?.next().unwrap();
    assert_eq!(row2.as_str(), "[123.456, -78]");

    let empty_row = MatrixParser::parse(Rule::row, "[]");
    assert_eq!(empty_row.is_err(), true);

    let invalid_row1 = MatrixParser::parse(Rule::row, "[123, 456");
    assert_eq!(invalid_row1.is_err(), true);

    let invalid_row2 = MatrixParser::parse(Rule::row, "[abc, 123]");
    assert_eq!(invalid_row2.is_err(), true);

    let invalid_row3 = MatrixParser::parse(Rule::row, "{456, 123}");
    assert_eq!(invalid_row3.is_err(), true);

    Ok(())
}

#[test]
fn test_matrix() -> Result<()> {
    let matrix1 = MatrixParser::parse(Rule::matrix, "[[1, 2, 3], [2, 4, 5]]")?.next().unwrap();
    assert_eq!(matrix1.as_str(), "[[1, 2, 3], [2, 4, 5]]");

    let matrix2 = MatrixParser::parse(Rule::matrix, "[[1, 2, 3], [2, 5]]")?.next().unwrap();
    assert_eq!(matrix2.as_str(), "[[1, 2, 3], [2, 5]]");

    let matrix3 = MatrixParser::parse(Rule::matrix, "[[1]]")?.next().unwrap();
    assert_eq!(matrix3.as_str(), "[[1]]");

    let invalid_matrix = MatrixParser::parse(Rule::matrix, "[123]");
    assert_eq!(invalid_matrix.is_err(), true);

    Ok(())
}

#[test]
fn test_mat_def() -> Result<()> {
    let mat_a = MatrixParser::parse(Rule::mat_def, "mat A = [[1, 2, 3], [2, 4, 5]]")?.next().unwrap();
    assert_eq!(mat_a.as_str(), "mat A = [[1, 2, 3], [2, 4, 5]]");

    let invalid_mat_a =  MatrixParser::parse(Rule::mat_def, "mat A = [[1, 2, 3] [2, 4, 5]]");
    assert_eq!(invalid_mat_a.is_err(), true);

    Ok(())
}

#[test]
fn test_add() -> Result<()> {
    let add1 = MatrixParser::parse(Rule::add, "add A, B")?.next().unwrap();
    assert_eq!(add1.as_str(), "add A, B");

    let invalid_add1 = MatrixParser::parse(Rule::add, "sub A, B");
    assert_eq!(invalid_add1.is_err(), true);

    let invalid_add2 = MatrixParser::parse(Rule::add, "add A, 3");
    assert_eq!(invalid_add2.is_err(), true);

    Ok(())
}

#[test]
fn test_sub() -> Result<()> {
    let sub1 = MatrixParser::parse(Rule::subtract, "sub A, B")?.next().unwrap();
    assert_eq!(sub1.as_str(), "sub A, B");

    let invalid_sub1 = MatrixParser::parse(Rule::subtract, "add A, B");
    assert_eq!(invalid_sub1.is_err(), true);

    let invalid_sub2 = MatrixParser::parse(Rule::subtract, "sub A, 3");
    assert_eq!(invalid_sub2.is_err(), true);

    Ok(())
}

#[test]
fn test_mult() -> Result<()> {
    let mult1 = MatrixParser::parse(Rule::mult, "mul A, B")?.next().unwrap();
    assert_eq!(mult1.as_str(), "mul A, B");

    let invalid_mult1 = MatrixParser::parse(Rule::mult, "add A, B");
    assert_eq!(invalid_mult1.is_err(), true);

    let invalid_mult2 = MatrixParser::parse(Rule::mult, "mult A, 3");
    assert_eq!(invalid_mult2.is_err(), true);

    Ok(())
}

#[test]
fn test_scale() -> Result<()> {
    let scale1 = MatrixParser::parse(Rule::scale, "scale A, 3")?.next().unwrap();
    assert_eq!(scale1.as_str(), "scale A, 3");

    let invalid_scale1 = MatrixParser::parse(Rule::scale, "add A, 3");
    assert_eq!(invalid_scale1.is_err(), true);

    let invalid_scale2 = MatrixParser::parse(Rule::scale, "scale A, B");
    assert_eq!(invalid_scale2.is_err(), true);

    Ok(())
}

#[test]
fn test_file() -> Result<()> {
    let file1 = r#"
    #Comment
    mat A = [[1, 2], [3, 4]]
    mat my_second_matrix = [[11, 12], [13, 14]]
    add A, my_second_matrix"#;

    MatrixParser::parse(Rule::file, file1)?;

    Ok(())
}