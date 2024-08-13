use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut vm = VirtualMachine::new();

    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == ".exit" {
            break;
        } else {
            let tokens = tokenize(input);
            match parse(tokens) {
                Ok(ast) => {
                    vm.execute(ast);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

enum Token {
    Select,
    Insert,
    Identifier(String),
    Value(String),
    Unknown,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for word in input.split_whitespace() {
        match word.to_uppercase().as_str() {
            "SELECT" => tokens.push(Token::Select),
            "INSERT" => tokens.push(Token::Insert),
            _ if word.starts_with('\'') && word.ends_with('\'') => {
                tokens.push(Token::Value(word.trim_matches('\'').to_string()))
            }
            _ => tokens.push(Token::Identifier(word.to_string())),
        }
    }
    tokens
}

enum ASTNode {
    Select(Vec<String>),
    Insert(String, String),
}

fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    if tokens.is_empty() {
        return Err("Empty command".into());
    }

    match tokens[0] {
        Token::Select => {
            let columns = tokens[1..]
                .iter()
                .filter_map(|t| match t {
                    Token::Identifier(id) => Some(id.clone()),
                    _ => None,
                })
                .collect();
            Ok(ASTNode::Select(columns))
        }
        Token::Insert => {
            if tokens.len() == 3 {
                if let (Token::Identifier(id), Token::Value(val)) = (&tokens[1], &tokens[2]) {
                    return Ok(ASTNode::Insert(id.clone(), val.clone()));
                }
            }
            Err("Invalid INSERT syntax".into())
        }
        _ => Err("Unrecognized command".into()),
    }
}

struct VirtualMachine {
    table: HashMap<String, Vec<String>>,
}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            table: HashMap::new(),
        }
    }

    fn execute(&mut self, node: ASTNode) {
        match node {
            ASTNode::Select(columns) => {
                for col in columns {
                    if let Some(values) = self.table.get(&col) {
                        for value in values {
                            println!("{}: {}", col, value);
                        }
                    } else {
                        println!("{}: NULL", col);
                    }
                }
            }
            ASTNode::Insert(id, value) => {
                self.table.entry(id).or_insert_with(Vec::new).push(value);
                println!("Inserted.");
            }
        }
    }
}

struct Row {
    id: String,
    value: String,
}

struct Table {
    rows: Vec<Row>,
}

impl Table {
    fn new() -> Self {
        Table { rows: Vec::new() }
    }

    fn insert(&mut self, id: String, value: String) {
        self.rows.push(Row { id, value });
    }

    fn select(&self, id: &str) -> Option<&Row> {
        self.rows.iter().find(|&row| row.id == id)
    }
}
use std::fs::{File, OpenOptions};
use std::io::{Read};

impl Table {
    fn save_to_file(&self, filename: &str) {
        let mut file = OpenOptions::new().write(true).create(true).open(filename).unwrap();
        for row in &self.rows {
            writeln!(file, "{},{}", row.id, row.value).unwrap();
        }
    }

    fn load_from_file(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let mut table = Table::new();
        for line in content.lines() {
            let mut parts = line.split(',');
            let id = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            table.insert(id, value);
        }
        table
    }
}
