use crate::storage::Database;
use crate::parser::SqlStatement;
use crate::errors::DatabaseError;

pub struct VirtualMachine {
    database: Database,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            database: Database::new(),
        }
    }
    
    pub fn execute(&mut self, statement: SqlStatement) -> Result<String, DatabaseError> {
        match statement {
            SqlStatement::CreateTable { table_name, columns } => {
                self.database.create_table(&table_name, columns)?;
                Ok(format!("Table {} created", table_name))
            }
            SqlStatement::Insert { table_name, values } => {
                self.database.insert(&table_name, values)?;
                Ok(format!("Inserted into {}", table_name))
            }
            SqlStatement::Select { table_name, columns, conditions } => {
                let results = self.database.select(&table_name, &columns, conditions)?;
                Ok(format!("Selected {} rows", results.len()))
            }
            SqlStatement::Delete { table_name, conditions } => {
                let deleted = self.database.delete(&table_name, conditions)?;
                Ok(format!("Deleted {} rows", deleted))
            }
        }
    }
}
