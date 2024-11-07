use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use qpdf::QPdf;
use lopdf::Document;

pub fn load_pdf<'a>(input_file: &'a str, password: &'a str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let doc = Document::load(input_file)?;
    match doc.is_encrypted() {
        true => {
            Ok(decrypt_pdf(input_file, password)?)
        }
        false => {
            let pdf = QPdf::read(input_file)?;
            Ok(pdf.writer().preserve_encryption(false).write_to_memory()?)
        }
    }
}

fn decrypt_pdf<'a>(input_file: &'a str, b64pw: &'a str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let decoded = BASE64_STANDARD.decode(b64pw)?;
    let pw = String::from_utf8(decoded)?;
    let doc = QPdf::read_encrypted(input_file, &pw)?;

    Ok(doc.writer().preserve_encryption(false).write_to_memory()?)
}

pub fn write_pdf(pdf: &Vec<u8>, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let doc = QPdf::read_from_memory(pdf)?;
    doc.writer().write(output_file)?;
    println!("Successfully decrypted and saved to {:?}", output_file);

    Ok(())
}