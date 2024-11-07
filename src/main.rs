use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use qpdf::QPdf;
use std::env;

fn decrypt_pdf(input_file: &str, b64pw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let decoded = BASE64_STANDARD.decode(b64pw)?;
    let pw = String::from_utf8(decoded)?;
    let doc = QPdf::read_encrypted(input_file, &pw)?;
    doc.writer().preserve_encryption(false).write("decrypted.pdf")?;
    println!("Successfully decrypted and saved to decrypted.pdf");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Config related ENV variables
    let input_file = env::var("INPUT_FILE").expect("$INPUT_FILE is not set");
    let b64pw = env::var("BASE64_PASSWORD").expect("$BASE64_PASSWORD is not set");

    decrypt_pdf(&input_file, &b64pw)?;
    Ok(())

    // 1. Open the pdf
    // 2. Decrypt the pdf
    // 3. Split PDF pages into structs
    //    - Page
    //    - Text
    //    - Slide
    //    - Book
}
// <</Filter /Standard/V 4/R 4/Length 128/CF <</StdCF <</Length 16/CFM /AESV2/AuthEvent /DocOpen>>>>/StmF /StdCF/StrF /StdCF/EncryptMetadata false/O <606ab61777f1a51b49bd84a24b1557fbdfbbf94484b5c8748e00a42148d3eea5>/U <c1eda677def53da95608a60a9f6365bf28bf4e5e4e758a4164004e56fffa0108>/P -3904>>
