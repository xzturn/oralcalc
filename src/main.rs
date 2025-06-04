use clap::Parser;

mod binary_op;
mod tree_node;
mod worksheet;
mod errors;

use worksheet::WorksheetGenerator;

/// A command-line tool for generating math worksheets with arithmetic problems
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of pages of A4 paper to print
    #[arg(short = 'n', long = "pages", default_value_t = 0)]
    pages: u32,
    
    /// Number of rows in an A4 paper
    #[arg(short = 'r', long = "rows", default_value_t = 18)]
    rows: u32,
    
    /// Number of columns in an A4 paper  
    #[arg(short = 'c', long = "cols", default_value_t = 3)]
    cols: u32,
    
    /// Number of white spaces between two columns
    #[arg(short = 'w', long = "wsep", default_value_t = 14)]
    wsep: u32,
    
    /// Enable debug mode
    #[arg(long = "debug", default_value_t = false)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.debug {
        println!("Debug mode enabled");
        println!("Configuration: pages={}, rows={}, cols={}, wsep={}", 
                args.pages, args.rows, args.cols, args.wsep);
    }
    
    let generator = WorksheetGenerator::new(args.rows, args.cols, args.wsep);
    generator.generate_pages(args.pages)?;
    
    Ok(())
}
