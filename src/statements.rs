// statements.rs:
//      Various state enums to help with error handling and routing
//      Includes functions to execute SQL commands: SELECT, INSERT

use crate::table::Table;
use crate::table::Row;

pub enum ExecuteResult{
    ExecuteSuccess,
    //ExecuteTableFull,
    ExecuteFailed
}

pub enum StatementType {
    StatementNothing,
    StatementInsert,
    StatementSelect,
}

pub enum PrepareResult {
    PrepareSuccess,
    //PrepareSyntaxError,
    PrepareUnrecognizedStatement
}

pub fn prepare_statement(command: &String) -> (PrepareResult, StatementType) {
    let parsed: Vec<&str> = command.trim().split_whitespace().collect::<Vec<_>>();
    let prep = PrepareResult::PrepareSuccess; 

    match parsed[0].to_lowercase().as_str() {
        "insert" => { 
            (prep, StatementType::StatementInsert)
        }

        "select" => { 
            (prep, StatementType::StatementSelect)
        }

        _ => { 

            (PrepareResult::PrepareUnrecognizedStatement, StatementType::StatementNothing)
        }
    }
}

// High level function to handle command routing
pub fn execute_statement(statement: &StatementType, table: &mut Table) -> ExecuteResult {
    match statement {
        StatementType::StatementInsert => {
            execute_insert(table)
        }
        StatementType::StatementSelect => {
            execute_select(table)
        }
        _ => {
            println!("ya goofd");
            ExecuteResult::ExecuteFailed
        }
    }
}

// TODO: add data param to actually put useful info into db
pub fn execute_insert(table: &mut Table) -> ExecuteResult {
    let data: Vec<&str> = vec!["Something", "test"];
    table.add_row(&data);
    ExecuteResult::ExecuteSuccess
}

// Works for now, eventually will need to return data
pub fn execute_select(table: &Table) -> ExecuteResult {

    for i in 0..table.rows.len() {
        let decoded: Row = bincode::deserialize(&table.rows[i]).unwrap();
        println!("{:?}", decoded);
    }
    ExecuteResult::ExecuteSuccess
}


