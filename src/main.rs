use clap::Parser;
use colored::Colorize;
use std::fs::{self, File};
use std::io::Read;

use crate::cli::Args;
use crate::parser::{parse_openapi, write_request_body_to_template};

mod cli;
mod log;
mod parser;
mod templates;

fn main() {
    let args = Args::parse();
    fs::create_dir_all(&args.request_out_dir)
        .expect("Failed to create or check the request output directory");
    fs::create_dir_all(&args.response_out_dir)
        .expect("Failed to create or check the response output directory");

    // read openapi file
    let mut file = File::open(&args.openapi_path).expect("Failed to open openapi file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Successfully opened openapi file but could not read from it");
    let data = contents;

    // parse openapi file into struct
    let openapi = parse_openapi(&data.to_string()).expect("Failed to parse openapi file");

    // generate files from openapi struct
    for (path, method, operation) in openapi.operations() {
        // if path == "/v1/companies" && method == "post" {
        if let Some(body) = &operation.request_body {
            println!("");
            println!("{}: {}", method.blue(), path.green());
            if let Some(request_body) = body.as_item() {
                match write_request_body_to_template(&openapi, request_body, &args.request_out_dir)
                {
                    Ok(_) => println!("Success"),
                    Err(err) => eprintln!("{}: {}", "Error".red(), err.to_string().red()),
                }
            }
        }
        // }
    }

    println!("Finished successfully");
}
