// Notes: 
//      Varchar will not have a limit, in rust this means it will be hardcoded when compiled.
//      This file contains meta command handling, CLI input and the high level while loop for the db.

use std::io::{self, Read, Write};

mod statements;
mod table;

use statements::ExecuteResult;
use statements::PrepareResult;
use statements::StatementType;
use statements::prepare_statement;
use statements::execute_statement;

use table::Table;

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

struct InputBuffer {
    buffer: String,
    input_length: usize
}

impl InputBuffer{ 
    fn update_length(&mut self, n: usize) {
        self.input_length = n;
    }

    fn update_input(&mut self, input: String) {
        self.buffer = input;
    }
}

// TODO: Read up on the rust way of handling user inputs
fn read_input() -> String {
    let mut input_buffer = InputBuffer {
        buffer: String::new(),
        input_length: 0
    };

    let mut input = String::new();
    let mut bytes_read: usize = 0;

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            bytes_read = n;
            input = input;
        }
        Err(error) => println!("Error reading input: {}", error)
    }

    bytes_read -= 1;
    input_buffer.update_length(bytes_read);
    input_buffer.update_input(input);
    input_buffer.buffer
}

// Responsible for meta commands such as exit, new db, ect
fn do_meta_command(command: &String) -> MetaCommandResult {
    match command.trim() {
        ".exit" => {
            println!("Exiting db");
            std::process::exit(123);
        }

        _ => {
            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    }
}

fn main() {
    let dummy_columns: Vec<String> = vec!["Username".to_string(), "Email".to_string()];

    // Creating a test struct
    let mut table = Table {
        num_rows: 0,
        pages: 0,
        columns: dummy_columns,
        rows: Vec::new(),
    };

    let done = false;

    while !done {
        print!("db > ");
        std::io::stdout().flush().unwrap();

        let command = read_input();

        if command.trim().chars().next().unwrap() == '.' {
            match do_meta_command(&command) {
                MetaCommandResult::MetaCommandSuccess => {
                    continue;
                }
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized meta command {}", command);
                    continue;
                }
            }
        } 

        let statement_status: (PrepareResult, StatementType) = prepare_statement(&command);

        // Pattern matching PrepareResult enum
        match statement_status.0 {
            PrepareResult::PrepareSuccess => {
                let statement = statement_status.1; // statement_status.1 == returned struct Statement
                let new_row: ExecuteResult = execute_statement(&statement, &mut table);
                
                match new_row {
                    ExecuteResult::ExecuteSuccess => {
                        println!("Command succeeded");
                    }
                    ExecuteResult::ExecuteFailed => {
                        println!("Command failed");
                    }
                    _ => {
                        println!(":thinking:");
                    }
                }
            }

            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement");
                continue;
            }

            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword: {}", command);
                continue;
            }
        };

// Might be redundant 
//        match execution_status. {
//            ExecuteResult::ExecuteSuccess => {
//                println!("Executed")
//                break;
//            }
//            ExecuteResult::ExecuteTableFull => {
//                println!("Error: Table full")
//                break;
//            }
//        }

    }
}