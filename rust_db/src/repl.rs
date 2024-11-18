use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::vm::VirtualMachine;

pub fn start_repl() -> anyhow::Result<()> {
    let mut rl = Editor::<()>::new()?;
    let mut vm = VirtualMachine::new();
    
    println!("Welcome to Custom Database REPL");
    
    loop {
        let readline = rl.readline("db> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    break;
                }
                
                rl.add_history_entry(line.as_str());
                
                match Tokenizer::tokenize(&line) {
                    Ok(tokens) => {
                        match Parser::parse(tokens) {
                            Ok(statement) => {
                                match vm.execute(statement) {
                                    Ok(result) => println!("{}", result),
                                    Err(e) => eprintln!("Execution error: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Parsing error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Tokenization error: {}", e),
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
    
    Ok(())
}