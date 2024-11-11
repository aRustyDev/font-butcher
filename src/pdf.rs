use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use qpdf::{QPdf,QPdfObjectLike};
// use pdf_extract::*;
// use std::error::Error;
// use std::io::ErrorKind;
// use crate::env;

// TODO: Remove Watermarks
// TODO: Decrypt in-memory
// TODO: Implement selective decryption (qpdf, lopdf, & pdf-extract)
// TODO: Implement encryption
// TODO: Implement selective encryption (qpdf, lopdf, & pdf-extract)

pub fn load_pdf<'a>(input_file: &'a str, b64_pw: Option<&'a str>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    // Decrypt the PDF if necessary & load it in-memory
    // Then Split the PDF into pages, and return the pages as a Vec<Vec<u8>>
    match b64_pw { // TODO: implement Some/None for env_vars["BASE64_PASSWORD"].as_str()
        Some(b64) => {
            // Decode the Password
            let decoded = BASE64_STANDARD.decode(b64)?;
            let pw = String::from_utf8(decoded)?;

            // Load the PDF from disk
            let encrypted = QPdf::read_encrypted(input_file, &pw)?;

            // Decrypt the PDF in-memory
            let bytes = encrypted.writer()
                            .preserve_encryption(false)
                            .write_to_memory()?;

            // Read the decrypted PDF from memory
            // Split the PDF into pages & return the pages as a Vec<Vec<u8>>
            Ok(convert_pages_to_vec(&bytes)?)
        }
        None => {
            // Load the PDF from disk
            let decrypted = QPdf::read(input_file)?;

            // Load the PDF in-memory
            let bytes = decrypted.writer()
                            .write_to_memory()?;

            // Split the PDF into pages & return the pages as a Vec<Vec<u8>>
            Ok(convert_pages_to_vec(&bytes)?)
        }
    }
}

fn convert_pages_to_vec(bytes: &Vec<u8>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let pdf = QPdf::read_from_memory(bytes)?;
    let pages = pdf.get_pages()?;
    Ok(pages.iter().map(|page| page.as_binary_string()).collect())
}

pub fn write_pdf(pdf: &Vec<u8>, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let doc = QPdf::read_from_memory(pdf)?;
    doc.writer().write(output_file)?;
    println!("Successfully decrypted and saved to {:?}", output_file);

    Ok(())
}

// fn unstamp_page(pdf: &Document, page: &u32, watermark: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
//     pdf.get_pages().into_iter().map(|id| 
//         match pdf.replace_text(*page, watermark, "") {
//             Ok(page) => match pdf.get_page_content(id).unwrap() {
//                 Ok(content) => Ok(content),
//                 Err(e) => Err(Box::new(e)),
//             }, 
//             Err(e) => Err(Box::new(e)),
//         }
//     ).collect();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_decryption() {
        let input_file = "encrypted.pdf";
        let b64pw = "c1eda677def53da95608a60a9f6365bf28bf4e5e4e758a4164004e56fffa0108";
        let result = load_pdf(input_file, b64pw);
        assert!(result.is_ok());
    }

    #[test]
    fn new_pdf() {
        let output_file = "test.pdf";
        let pdf: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = write_pdf(&pdf, output_file);
        assert!(result.is_ok());
    }
}