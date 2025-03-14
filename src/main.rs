mod parser;
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    
    let mut parser = Parser::new(&filename);
    let mpng = parser.parse();
    
    mpng.print();

}
