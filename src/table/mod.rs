//use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Table {
    pub num_rows: u32,
    pub pages: u32,
    pub columns: Vec<String>,
    pub rows: Vec<Row>
}

// Todo possibly create a struct that holds column name and data. 
impl Table {
    pub fn add_row(&mut self, data: &Vec<&str>) {
        let new_row = Row {
            row_number: self.num_rows,
            id: "lkajsdflkajdfs".to_string(),
            username: "Alec".to_string(),
            email: "owo@somethingweeb.com".to_string()
        };

        self.rows.push(new_row);
        self.num_rows += 1;
    }
}

#[derive(Debug)]
pub struct Row {
    pub row_number: u32,
    pub id: String,
    pub username: String,
    pub email: String
}