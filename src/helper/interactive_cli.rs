use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[allow(dead_code)]
pub fn ask_single_line(question: &str) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    let readline = rl.readline(format!("{}: ", question).as_str());
    match readline {
        Ok(line) => {
            // rl.add_history_entry(line.as_str());
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

#[allow(dead_code)]
pub fn select_from_list(question: &str, list: &[&str], allow_default: bool) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    let mut index = 0;
    let mut selected = String::from("");
    if allow_default {
        println!("{} (default: {}): ", question, list[0])
    } else {
        println!("{}: ", question)
    };
    for item in list {
        println!("{}: {}", index, item);
        index += 1;
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // rl.add_history_entry(line.as_str());
                let line = line.trim();
                if line.is_empty() {
                    if allow_default {
                        selected = String::from(list[0]);
                        break;
                    }
                    continue;
                }
                let line = line.parse::<usize>();
                match line {
                    Ok(line) => {
                        if line < list.len() {
                            selected = String::from(list[line]);
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

// not working
#[allow(dead_code)]
fn select_from_list_with_arrow_key(question: &str, choices: &[&str]) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    let mut cursor = 0;
    println!("{}: ", question);
    loop {
        for (i, choice) in choices.iter().enumerate() {
            if i == cursor {
                println!("> {}", choice);
            } else {
                println!("  {}", choice);
            }
        }
        let readline = rl.readline("");
        match readline {
            Ok(line) => {
                match line.as_ref() {
                    "\u{1B}[A" => { // Up arrow
                        if cursor > 0 {
                            cursor -= 1;
                        }
                    },
                    "\u{1B}[B" => { // Down arrow
                        if cursor < choices.len() - 1 {
                            cursor += 1;
                        }
                    },
                    "" | "\r" | "\n" | "\r\n" => { // Enter key
                        return Ok(choices[cursor].to_string());
                    },
                    _ => {
                        println!("Selection: `{}`", line);
                    }
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}

