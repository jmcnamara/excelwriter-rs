// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;
    worksheet.write(6, 2, "Bar")?;
    worksheet.write(13, 6, "Baz")?;

    let note = Note::new("Some text").add_author_prefix(false);
    worksheet.insert_note(0, 0, &note)?;
    worksheet.insert_note(0, 3, &note)?;
    worksheet.insert_note(6, 2, &note)?;
    worksheet.insert_note(9, 4, &note)?;
    worksheet.insert_note(13, 6, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_comment16() {
    let test_runner = common::TestRunner::new()
        .set_name("comment16")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
