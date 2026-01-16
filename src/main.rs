use std::{fs::{self, File}, io::{self, Write}};
mod let_expr;
mod type_expr;
use let_expr::parse_let_func;

pub enum ExprType {
    Let,
    Loop,
    Func
}

fn expr_type(command: &str) -> ExprType {
    if command.starts_with("пусть") {
        ExprType::Loop
    } else if command.starts_with("для") { 
        ExprType::Loop
    } else{
        ExprType::Func
    }
}

fn parse_file(text: String, output: &mut File) -> io::Result<()> {
    for line in 
            text.lines()
            .filter(|line| !line.is_empty() && !line.starts_with(['/', '!']))
            .map(&str::trim) {
        match expr_type(line) {
            ExprType::Let =>{

            }
            _=> todo!()
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("{}", parse_let_func("пусть имя: Нат      =считать!(\"let me know your age\")"));
    return Ok(());
    let filename = "Алгоритм.абв";
    let new_filename = 
        format!("{}.rs", filename.strip_suffix(".абв").unwrap());

    let text = fs::read_to_string(filename)?;
    let mut file = File::create(new_filename)?;
    writeln!(file, "#![allow(unused)]\nuse std::io;")?; 
    Ok(())
}
