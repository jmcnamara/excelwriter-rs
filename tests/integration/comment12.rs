// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;

    let note = Note::new("Some text");
    worksheet.insert_note(0, 0, &note)?;

    worksheet.set_row_height(0, 21)?;
    worksheet.set_column_width(1, 10)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_comment12() {
    let test_runner = common::TestRunner::new()
        .set_name("comment12")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
