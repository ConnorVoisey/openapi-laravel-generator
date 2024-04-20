use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to output requests to
    #[arg(long, default_value_t = String::from("output/requests"))]
    pub request_out_dir: String,

    /// Directory to output responses to
    #[arg(long, default_value_t = String::from("output/responses"))]
    pub response_out_dir: String,

    /// Path to openapi file
    #[arg(long, default_value_t = String::from("openapi.yaml"))]
    pub openapi_path: String,

    /// Logs more output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
