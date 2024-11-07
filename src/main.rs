// use qpdf::QPdf;
use std::env;
// use crate::unstamp;
mod pdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Config related ENV variables
    let input_file = env::var("INPUT_FILE").expect("$INPUT_FILE is not set");
    let b64pw = env::var("BASE64_PASSWORD").expect("$BASE64_PASSWORD is not set");

    // Open & decrypt the pdf
    let pdf = pdf::load_pdf(&input_file, &b64pw)?;

    // Remove Watermarks
    // unstamp::unstamp(&pdf)?;

    // Manage per page data -> DuckDB

    // Write the finished pdf
    pdf::write_pdf(&pdf, &input_file)?;

    Ok(())
}
// <</Filter /Standard/V 4/R 4/Length 128/CF <</StdCF <</Length 16/CFM /AESV2/AuthEvent /DocOpen>>>>/StmF /StdCF/StrF /StdCF/EncryptMetadata false/O <606ab61777f1a51b49bd84a24b1557fbdfbbf94484b5c8748e00a42148d3eea5>/U <c1eda677def53da95608a60a9f6365bf28bf4e5e4e758a4164004e56fffa0108>/P -3904>>
