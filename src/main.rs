mod pdf;
mod utils;
mod env;

// use pdf_extract::*;

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

    // [x] Open & decrypt the pdf
    match &env_vars["INPUT_FILE"] {
        Some(pdf) => {
            match &env_vars["BASE64_PASSWORD"] {
                Some(b64) => {
                    let pdf = pdf::load_pdf(&pdf, Some(&b64))?;
                }
                None => {
                    let pdf = pdf::load_pdf(&pdf, None)?;
                }
            }
        },
        None => { // TODO: Handle missing env vars
            println!("input_file: None");
        }
    }

    // [ ] Pull out text from PDF
    // for page in pdf.iter() {
    //     let out = extract_text_from_mem(&page).unwrap();
    //     println!("pages extracted: {:?}", out);
    // }

    // [ ] Remove Watermarks

    // [ ] Parse data into database -> DuckDB

    // [ ] Parse data from database -> Markdown

    // [ ] Parse data from database -> HTML

    // [ ] Parse data from database -> Slides / sli.dev

    Ok(())
}
// <</Filter /Standard/V 4/R 4/Length 128/CF <</StdCF <</Length 16/CFM /AESV2/AuthEvent /DocOpen>>>>/StmF /StdCF/StrF /StdCF/EncryptMetadata false/O <606ab61777f1a51b49bd84a24b1557fbdfbbf94484b5c8748e00a42148d3eea5>/U <c1eda677def53da95608a60a9f6365bf28bf4e5e4e758a4164004e56fffa0108>/P -3904>>
