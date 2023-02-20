use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
pub fn ask_single_line(question: &str) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    println!("{}: ", question);
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str());
            Ok(line)
        },
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            Ok("".to_string())
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            Ok("".to_string())
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Ok("".to_string())
        }
    }
}

pub fn select_from_list(question: &str, list: &Vec<String>) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    let mut index = 0;
    let mut selected = String::from("");
    for item in list {
        println!("{}: {}", index, item);
        index += 1;
    }
    loop {
        let readline = rl.readline(&format!("{}: ", question));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let line = line.parse::<usize>();
                match line {
                    Ok(line) => {
                        if line < list.len() {
                            selected = list[line].clone();
                            break;
                        } else {
                            println!("Invalid selection");
                        }
                    }
                    Err(_) => {
                        println!("Invalid selection");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(selected)
}

