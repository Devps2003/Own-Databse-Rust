use std::collections::HashMap;
use crate::errors::DatabaseError;

pub struct Database {
    tables: HashMap<String, Table>,
}

pub struct Table {
    columns: Vec<(String, String)>,
    data: Vec<Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }
    
    pub fn create_table(&mut self, name: &str, columns: Vec<(String, String)>) -> Result<(), DatabaseError> {
        if self.tables.contains_key(name) {
            return Err(DatabaseError::StorageError("Table already exists".to_string()));
        }
        
        self.tables.insert(name.to_string(), Table {
            columns,
            data: Vec::new(),
        });
        
        Ok(())
    }
    
    pub fn insert(&mut self, table_name: &str, values: Vec<String>) -> Result<(), DatabaseError> {
        let table = self.tables.get_mut(table_name)
            .ok_or_else(|| DatabaseError::StorageError("Table not found".to_string()))?;
        
        if values.len() != table.columns.len() {
            return Err(DatabaseError::StorageError("Incorrect number of values".to_string()));
        }
        
        table.data.push(values);
        Ok(())
    }
    
    pub fn select(&self, table_name: &str, _columns: &[String], _conditions: Option<String>) -> Result<Vec<Vec<String>>, DatabaseError> {
        let table = self.tables.get(table_name)
            .ok_or_else(|| DatabaseError::StorageError("Table not found".to_string()))?;
        
        Ok(table.data.clone())
    }
    
    pub fn delete(&mut self, table_name: &str, _conditions: Option<String>) -> Result<usize, DatabaseError> {
        let table = self.tables.get_mut(table_name)
            .ok_or_else(|| DatabaseError::StorageError("Table not found".to_string()))?;
        
        let deleted_count = table.data.len();
        table.data.clear();
        
        Ok(deleted_count)
    }
}