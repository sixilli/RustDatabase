use crate::table::Table;

pub enum ExecuteResult{
    ExecuteSuccess,
    ExecuteTableFull,
    ExecuteFailed
}

pub enum StatementType {
    StatementNothing,
    StatementInsert,
    StatementSelect,
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareUnrecognizedStatement
}

pub fn execute_insert(table: &mut Table) -> ExecuteResult {
    //if table.NumRows >= MAX_ROWS {
        //ExecuteResult::ExecuteTableFull
    //} else {
    let data: Vec<&str> = vec!["Something", "test"];
    table.add_row(&data);
    //let serialized = serde_json::to_string(row).unwrap();
    //serialized
        //ExecuteResult::ExecuteSuccess
    //}
    ExecuteResult::ExecuteSuccess
}

pub fn execute_select(table: &Table) -> ExecuteResult {
    for i in 0..table.rows.len() {
        //let entry: Row = serde_json::from_str(row).unwrap();
        println!("{:?}", table.rows[i]);
    }
    ExecuteResult::ExecuteSuccess
}

pub fn prepare_statement(command: &String) -> (PrepareResult, StatementType) {
    let parsed: Vec<&str> = command.trim().split_whitespace().collect::<Vec<_>>();
    let prep = PrepareResult::PrepareSuccess; 

    match parsed[0].to_lowercase().as_str() {
        "insert" => { 
            let id: String = parsed[1].to_string(); //.parse::<u32>().unwrap();
            let user: String = parsed[2].to_string();
            let email: String = parsed[3].to_string();

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