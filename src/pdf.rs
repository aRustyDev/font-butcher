use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use qpdf::{QPdf,QPdfObjectLike};
use std::collections::BTreeMap;
use regex::Regex;

// TODO: Remove Watermarks
// TODO: Decrypt in-memory
// TODO: Implement selective decryption (qpdf, lopdf, & pdf-extract)
// TODO: Implement encryption
// TODO: Implement selective encryption (qpdf, lopdf, & pdf-extract)

#[derive(Debug)]
pub struct Metadata<'a> {
    pub stamps: Vec<String>,
    pub book_number: String,
    pub cert: String,
    pub book_title: String,
    pub course_number: String,
    pub course_title: String,
    pub course_release: String,
    pub bytes: &'a [u8],
    pub pages: Vec<String>,
}

pub fn load_pdf(input_file: &str, b64_pw: Option<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // pub fn load_pdf(input_file: &str, pgs: &mut Pages, b64_pw: Option<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
            // pgs.parse_pdf(&bytes).unwrap();
            Ok(bytes)
        }
        None => {
            // Load the PDF from disk
            let decrypted = QPdf::read(input_file)?;

            // Load the PDF in-memory
            let bytes = decrypted.writer()
                            .write_to_memory()?;

            // Split the PDF into pages & return the pages as a Vec<Vec<u8>>
            // pgs.parse_pdf(&bytes).unwrap();
            Ok(bytes)
        }
    }
}

pub fn write_pdf(pdf: &Vec<u8>, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let doc = QPdf::read_from_memory(pdf)?;
    doc.writer().write(output_file)?;
    println!("Successfully decrypted and saved to {:?}", output_file);

    Ok(())
}

impl Metadata<'_> {
    pub fn new(bytes: &[u8]) -> Result<Metadata, Box<dyn std::error::Error>> {
        Ok(Metadata {
            stamps: vec![],
            book_number: String::new(),
            cert: String::new(),
            book_title: String::new(),
            course_number: String::new(),
            course_title: String::new(),
            course_release: String::new(),
            bytes: bytes,
            pages: Vec::<String>::new(),
        })
    }
    pub fn process_pdf(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let text = pdf_extract::extract_text_from_mem(&self.bytes).unwrap();
        let by_page = Regex::new(r"(Licensed To: .+ \d{1,2}, 20\d{2})").unwrap();

        self.pages = by_page.split(&text).into_iter().map(|x| x.to_string()).filter(|x| x!="\n").collect::<Vec<_>>();

        let title_page = self.pages[0].split("\n\n").collect::<Vec<&str>>();
        let pg_two = self.pages[1].split("\n\n").collect::<Vec<&str>>();
    
        self.stamps = vec![title_page[6].to_string(), title_page[7].to_string(), title_page[8].to_string(), title_page[9].to_string(), title_page[10].to_string(), title_page[11].to_string(), title_page[12].to_string()];
        self.book_number = title_page[4].to_string();
        self.cert = title_page[3].to_string();
        self.book_title = title_page[5].to_string();
        self.course_number = title_page[2].split(" | ").collect::<Vec<&str>>()[0].trim().to_string();
        self.course_title = title_page[2].split(" | ").collect::<Vec<&str>>()[1].trim().to_string(); 

        let course_release_re = format!("({}{})", self.course_number, "_\\S+");
        for obj in pg_two {
            let re = Regex::new(&course_release_re).unwrap();
            if re.is_match(&obj) {
                let caps = re.captures(&obj).unwrap();
                println!("caps: {:?}", (&caps.get(0).unwrap().as_str()).to_string());
                self.course_release = (&caps.get(0).unwrap().as_str()).to_string();
                break;
            }
        }
        
        Ok(())
    }
    
    pub fn strip_watermarks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.pages[1] = self.pages[1].replace(&self.course_release, "");
        // Loop through each page (as mutable reference)
        for page in self.pages.iter_mut() {
            // Loop through each stamp (as &str)
            for stamp in &self.stamps {
                // Replace each stamp occurrence with an empty string
                let formatted = format!("{}{}", stamp, " ");
                *page = page.replace(&formatted, "");
            }
        }

        Ok(())
    }
}

pub struct Pages {
    pub table: BTreeMap<u32, (usize, usize)>
}

impl Pages {
    pub fn new() -> Self {
        let table: BTreeMap<u32, (usize, usize)> = BTreeMap::<u32, (usize, usize)>::new();
        Pages { 
            table: table 
        }
    }
    
    // Safe method to read data between addresses
    pub fn read_range<'a>(&self, bytes: &'a Vec<u8>, start: usize, end: usize) -> Option<&'a [u8]> {
        if true && true {
            return Some(&bytes.as_slice()[start..end]);
        } else {
            None
        }
    }

    fn parse_pdf(&mut self, bytes: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let pdf = QPdf::read_from_memory(bytes)?;
        let pages = pdf.get_pages()?;
        let mut count: u32 = 0;
        for page in pages.iter() {
            let pg = page.as_binary_string();
            println!("len: {}", pg.len());
            self.get_addrs(&pg, &count)?;
            count += 1;
        }
        Ok(())
    }

    fn get_addrs(&mut self, bytes: &Vec<u8>, page: &u32) -> Result<(), Box<dyn std::error::Error>> {
        
        let slice = bytes.as_slice();
        let start = slice.as_ptr() as usize;
        let end = start + bytes.len() - 1;
        println!("pg[{}] | start: {:?}, end: {:?}, len: {:?}", page, start, end, slice.len());
        self.table.insert(*page, (start, end));
    
        Ok(())
    }
}

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