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
        // Removed args.cols from the debug output as it no longer exists
        println!("Configuration: pages={}, rows={}, wsep={}",
                args.pages, args.rows, args.wsep);
        println!("Note: The worksheet generator uses a fixed 3-column layout per row.");
    }

    // Pass only rows and wsep, as cols is no longer part of WorksheetGenerator::new
    // and also removed from Args
    let generator = WorksheetGenerator::new(args.rows, args.wsep);
    generator.generate_pages(args.pages)?;

    Ok(())
}

