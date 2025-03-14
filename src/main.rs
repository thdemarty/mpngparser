mod tests;
mod parser;
mod mpng;

use std::path::Path;

use clap::Parser as ArgParser;
use parser::Parser;


#[derive(ArgParser)]
#[command(version, about = "Yet another parser for minipng", long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(String))]
    filepath: String
}

fn main() {
    let args = Args::parse();
    let filepath = Path::new(&args.filepath);
    let mut parser = Parser::new(filepath);
    let mpng = parser.parse();
    println!("{}", mpng);
}
