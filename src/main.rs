use std::{env, fs, io::{self, Write}};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();  
    if args.len() < 2 {
        eprintln!("usage: [FILE NAME]");
        return Ok(())
    };
    let infile =  fs::read_to_string(&args[1])?;
    let mut outfile = 
        if let Some(name) = &args[1].strip_suffix(".абв") {
        fs::File::create(format!("{}.rs", name))?
    } else {
        eprintln!("can't find the name of the file (it can be ***.абв)");
        return Ok(())
    };
    writeln!(outfile, "#![allow(unused)]")?;
    writeln!(outfile, "use std::io;")?;
    return Ok(());
    parse_file(&infile, &mut outfile) 
}

fn parse_file(
    text: &str,
    outfile: &mut fs::File ) -> io::Result<()> {
    //parse - DONE
    //parse read 
    //parse write
    for line in text.lines().filter(|line|
        !line.is_empty() && !line.starts_with(['!', '/'])) {
    } 
    Ok(())
}

fn eq<'a>(
    arg: &'a str, 
    outfile: &mut fs::File) -> io::Result<(Option<&'a str>, Option<&'a str>)> {
    if let Some((first, second)) 
            = arg.split_once('=')
    {
        let first = match_keyword(first);
        write!(outfile, "{} =", first)?;
        return Ok((
            if !first.is_empty() {Some(first)} else {None}, 
            if !second.is_empty() {Some(second)} else {None}));
    }
    Ok((None, None))
}

fn match_keyword(key: &str) -> &str {
    if key.is_empty() {
        return "";
    }
    match key {
        "Алгоритм" =>   "fn",
        "Начало" =>     "{",
        "написать!" =>  "println!",
        "Конец" =>      "}",
        "пусть" =>      "let mut",
        "Строка" =>     "String",
        "Главная" =>    "main() -> io::Result<()>",
        "Новая" =>      "new",
        _=> key 
    }
}
