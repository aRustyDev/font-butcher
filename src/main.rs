// #![crate_name = "doc"]

mod pdf;
mod utils;
mod env;

use pdf_extract::*;
use lopdf::content::Content;

// TODO: Implement TUI / CLI
// TODO: Implement Logging
// TODO: Implement Error Handling
// TODO: Implement Loading Multiple PDFs
// TODO: Implement Loading PDFs from a URL
// TODO: Implement Loading PDFs from a directory
// TODO: Implement Loading PDF strings to a database

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [x] Get Config related ENV variables
    let env_vars = env::get_env_vars()?;
    // let mut pages = pdf::Pages::new();  
    let bytes: Vec<u8>;

    // [x] Open & decrypt the pdf
    match &env_vars["INPUT_FILE"] {
        Some(pdf) => {
            match &env_vars["BASE64_PASSWORD"] {
                Some(b64) => {
                    bytes = pdf::load_pdf(&pdf, Some(&b64))?;
                }
                None => {
                    bytes = pdf::load_pdf(&pdf, None)?;
                }
            }
        },
        None => { // TODO: Handle missing env vars
            panic!("No input_file provided");
        }
    }
    
    // [ ] Pull out text from PDF
    let mut md = pdf::Metadata::new(&bytes)?;
    md.process_pdf()?;
    md.strip_watermarks()?;

    println!("pg count: {:?}", md.pages.len());
    println!("pg2: {:?}", md.pages[1]);
    md.bytes = &[];
    md.pages = vec![];
    println!("md: {:?}", md);


    // [ ] Remove Watermarks

    // [ ] Parse data into database -> DuckDB

    // [ ] Parse data from database -> Markdown

    // [ ] Parse data from database -> HTML

    // [ ] Parse data from database -> Slides / sli.dev

    Ok(())
}
