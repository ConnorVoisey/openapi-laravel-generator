use std::fs;

use crate::parser::write_request_body_to_template;
use openapiv3::OpenAPI;

mod parser;
mod templates;

const REQUEST_OUT_DIR: &str = "./output/requests";
const RESPONSE_OUT_DIR: &str = "./output/responses";

fn main() {
    fs::create_dir_all(REQUEST_OUT_DIR)
        .expect("Failed to create or check the request output directory");
    fs::create_dir_all(RESPONSE_OUT_DIR)
        .expect("Failed to create or check the response output directory");

    let data = include_str!("../examples/openapi.yaml");
    let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
    for (path, method, operation) in openapi.operations() {
        // if path == "/v1/companies" && method == "post" {
        if let Some(body) = &operation.request_body {
            println!("path: {path}, method: {method}");
            if let Some(request_body) = body.as_item() {
                match write_request_body_to_template(&openapi, request_body, REQUEST_OUT_DIR) {
                    Ok(_) => println!("Success"),
                    Err(err) => eprintln!("ERRoR: {err}"),
                }
            }
        }
        // }
    }

    println!("Finished successfully");
}
