use crate::tokenizer::Token;
use crate::errors::DatabaseError;

#[derive(Debug)]
pub enum SqlStatement {
    CreateTable {
        table_name: String,
        columns: Vec<(String, String)>,
    },
    Insert {
        table_name: String,
        values: Vec<String>,
    },
    Select {
        table_name: String,
        columns: Vec<String>,
        conditions: Option<String>,
    },
    Delete {
        table_name: String,
        conditions: Option<String>,
    },
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<SqlStatement, DatabaseError> {
        match tokens.get(0) {
            Some(Token::Keyword(keyword)) => match keyword.to_uppercase().as_str() {
                "CREATE" => Parser::parse_create_table(&tokens),
                "INSERT" => Parser::parse_insert(&tokens),
                "SELECT" => Parser::parse_select(&tokens),
                "DELETE" => Parser::parse_delete(&tokens),
                _ => Err(DatabaseError::ParseError("Unknown statement".to_string())),
            },
            _ => Err(DatabaseError::ParseError("Invalid statement".to_string())),
        }
    }
    
    fn parse_create_table(tokens: &[Token]) -> Result<SqlStatement, DatabaseError> {
        // Look for table name after CREATE TABLE
        if tokens.len() < 3 {
            return Err(DatabaseError::ParseError("Incomplete CREATE TABLE statement".to_string()));
        }
        
        let table_name = match &tokens[2] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DatabaseError::ParseError("Invalid table name".to_string())),
        };
        
        Ok(SqlStatement::CreateTable {
            table_name,
            columns: vec![
                ("id".to_string(), "INTEGER".to_string()),
                ("name".to_string(), "STRING".to_string())
            ],
        })
    }
    
    fn parse_insert(tokens: &[Token]) -> Result<SqlStatement, DatabaseError> {
        // Expect pattern: INSERT INTO table_name VALUES (val1, val2, ...)
        if tokens.len() < 6 {
            return Err(DatabaseError::ParseError("Incomplete INSERT statement".to_string()));
        }
        
        let table_name = match &tokens[2] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DatabaseError::ParseError("Invalid table name".to_string())),
        };
        
        // Find the start of values list (after VALUES keyword)
        let values_start = tokens.iter().position(|t| 
            matches!(t, Token::Keyword(k) if k.to_uppercase() == "VALUES")
        ).ok_or_else(|| DatabaseError::ParseError("Missing VALUES keyword".to_string()))?;
        
        // Look for opening parenthesis
        let paren_start = tokens[values_start..].iter().position(|t| 
            matches!(t, Token::Punctuation('('))
        ).ok_or_else(|| DatabaseError::ParseError("Missing opening parenthesis".to_string()))?;
        
        // Collect values
        let mut values = Vec::new();
        let start_index = values_start + paren_start + 1;
        
        for token in &tokens[start_index..] {
            match token {
                Token::Literal(val) => values.push(val.clone()),
                Token::Punctuation(',') => continue,
                Token::Punctuation(')') => break,
                _ => return Err(DatabaseError::ParseError("Invalid value in INSERT".to_string())),
            }
        }
        
        Ok(SqlStatement::Insert {
            table_name,
            values,
        })
    }
    
    fn parse_select(tokens: &[Token]) -> Result<SqlStatement, DatabaseError> {
        // Expect pattern: SELECT * FROM table_name
        if tokens.len() < 4 {
            return Err(DatabaseError::ParseError("Incomplete SELECT statement".to_string()));
        }
        
        let table_name = match &tokens[3] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DatabaseError::ParseError("Invalid table name".to_string())),
        };
        
        Ok(SqlStatement::Select {
            table_name,
            columns: vec!["*".to_string()],
            conditions: None,
        })
    }
    
    fn parse_delete(tokens: &[Token]) -> Result<SqlStatement, DatabaseError> {
        // Expect pattern: DELETE FROM table_name
        if tokens.len() < 3 {
            return Err(DatabaseError::ParseError("Incomplete DELETE statement".to_string()));
        }
        
        let table_name = match &tokens[2] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DatabaseError::ParseError("Invalid table name".to_string())),
        };
        
        Ok(SqlStatement::Delete {
            table_name,
            conditions: None,
        })
    }
}