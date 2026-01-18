use std::{env, fs::{self, File}, io::{self, Write}};
mod let_expr;
mod type_expr;
mod if_expr;
mod write_expr;
mod loop_expr;
mod func_expr;
use func_expr::parse_func_expr;
use loop_expr::parse_loop_expr;
use write_expr::parse_write_expr;
use if_expr::parse_if_expr;
use let_expr::parse_let_expr;

fn parse_expr(line: &str, output: &mut File) -> io::Result<bool> {
    if line.starts_with("Алгоритм") {
        write!(output, "fn ")?; 
        if let Some(name) = line.split_whitespace().nth(1) {
            if name == "Главная" {
                writeln!(output, "main()")?;
            } else {
                writeln!(output, "{}", name)?;
            }
        }
    } else if line.starts_with("начало") || line.starts_with("Начало") {
        writeln!(output, "{{")?; 
    } else if line.starts_with("конец") || line.starts_with("Конец"){
            writeln!(output, "}}")?;
    } else if line.starts_with("вернуть") {
        if let Some((_, args)) = line.split_once(' ') {
            writeln!(output, "return {}", args)?;     
        }
    } 
    else if line.starts_with("сломать") {
        writeln!(output, "break;")?;
    } else if line.starts_with("пусть") {
        writeln!(output, "{}", parse_let_expr(line))?;
    } else if line.starts_with("если") || line.starts_with("иначе") {
        let (parsed, contin) = parse_if_expr(line);
        writeln!(output, "{}", parsed)?;
        return Ok(contin)
        
    } else if line.starts_with("написать") || line.starts_with("вывести") {
        writeln!(output, "{}", parse_write_expr(line))?;
    } else if line.starts_with("пока") || line.starts_with("для") {
        let (parsed, contin) = parse_loop_expr(line);
        writeln!(output, "{}", parsed)?;
        return Ok(contin);
    } else {
        writeln!(output, "{}", line)?;
    }
    Ok(false) 
}

fn requr<'a>(output: &mut File, line: &mut impl Iterator<Item = &'a str>) -> io::Result<()> {
    if let Some(next) = line.next() { 
        if parse_expr(next, output)? {
            requr(output, line)?;  
        } 
        writeln!(output, "}}")?;
    }
    Ok(())
}

fn parse_file(text: String, mut output: File) -> io::Result<()> {
    let mut iter = 
            text.lines()
            .filter(|line| !line.is_empty() && !line.starts_with(['/', '!']))
            .map(&str::trim);
    while let Some(line) = iter.next()
    {
        if parse_expr(line, &mut output)? {
            requr(&mut output, &mut iter)?; 
        } 
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("использование: cargo run -- [ИМЯ ФАЙЛА].абв");
        return Ok(())
    }
    let filename = &args[1];
    let new_filename = 
        format!("{}.rs", filename.strip_suffix(".абв").unwrap());

    let text = fs::read_to_string(filename)?;
    let mut file = File::create(new_filename)?;
    writeln!(file, "\
    #![allow(unused)]
    #![allow(warnings)]
    use std::{{
        fmt,
        str::FromStr,
        io::{{self, Write}}
    }};
    \n\n
    fn read_console<T: FromStr>(prompt: &str) -> T
    where T::Err: fmt::Debug
    {{
    print!(\"{{}}\", prompt);
    let _ = io::stdout().flush();
    let mut to_read = String::new();
    io::stdin().read_line(&mut to_read).expect(\"не могу считать строку\");
    to_read.parse::<T>().expect(\"несоответствие типов ввода\")
    }}\n\n")?; 

    parse_file(text, file)
}
