# Font Butcher

A tool to transform PDFs into data structures for further processing. It is a combination of a [qpdf](https://github.com/qpdf/qpdf) wrapper and a refactored version of [pdf-unstamper](https://github.com/hwding/pdf-unstamper) by [@hwding](https://github.com/hwding).

## How it works

1. Read PDF
2. Decrypt PDF
3. Remove Watermarks
4. Handle per page data -> store in DuckDB
5. Write PDF (cleaned w/o watermarks & not encrypted)
6. Process data in DuckDB
   1. Generate Word Cloud
   2. Generate Complete Index (minus filler words)
      1. Word count & location
   3. Generate Embeddings for the collection of books

## Build Dependencies (MacOS)

- libqpdf `brew install qpdf`

## Reference Material

- [A Breakdown Of How Lopdf Reads Pdfs](https://martyjon.es/2021/05/08/A-breakdown-of-how-lopdf-reads-PDFs.html)
- [The Structure of a PDF File](https://medium.com/@jberkenbilt/the-structure-of-a-pdf-file-6f08114a58f6)
- [PDF syntax 101: Understanding PDF Object Types](https://www.nutrient.io/blog/pdf-syntax-101/)
- [Developing with PDF (book)](https://learning.oreilly.com/library/view/developing-with-pdf/9781449327903/ch01.html)
- [Understanding PDF Vulnerabilities and Shellcode Attacks](https://www.infosecinstitute.com/resources/hacking/pdf-file-format-basic-structure/)
- [Abbyy: Types of PDFs](https://pdf.abbyy.com/learning-center/pdf-types/)
- [Adobe: PDF File types](https://www.adobe.com/uk/acrobat/resources/document-files/pdf-types.html)
- [PDF Object Types](https://labs.appligent.com/appligent-labs/pdfblog/pdf-object-types)
- [Adobe: How to make an interactive PDF](https://www.adobe.com/acrobat/hub/how-to-make-a-pdf-interactive.html)
- [Interactive PDF Guide](https://flippingbook.com/blog/marketing-tips/interactive-pdf-ideas)
- [GraVoc: How to make an interactive PDF](https://www.gravoc.com/2020/07/02/how-to-make-an-interactive-pdf/)
- [PDF Explained (book)](https://learning.oreilly.com/library/view/pdf-explained/9781449321581/ch02.html#id423152)
