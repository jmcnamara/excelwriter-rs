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

    let _worksheet1 = workbook.add_worksheet();
    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_default_note_author("John");

    worksheet2.write(0, 0, "Foo")?;

    let note = Note::new("Some text");
    worksheet2.insert_note(1, 1, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_comment11() {
    let test_runner = common::TestRunner::new()
        .set_name("comment11")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
