// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;

    let format = Format::new()
        .set_background_color("#6EE44E")
        .set_font_name("Tahoma")
        .set_font_size(8);

    let note = Note::new("Some text")
        .add_author_prefix(false)
        .set_format(format);
    worksheet.insert_note(1, 1, &note)?;

    workbook.save(filename)?;

    Ok(())
}

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;

    let note = Note::new("Some text")
        .add_author_prefix(false)
        .set_background_color("#6EE44E");
    worksheet.insert_note(1, 1, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_comment10_1() {
    let test_runner = common::TestRunner::new()
        .set_name("comment10")
        .unique("1")
        .set_function(create_new_xlsx_file_1)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_comment10_2() {
    let test_runner = common::TestRunner::new()
        .set_name("comment10")
        .unique("2")
        .set_function(create_new_xlsx_file_2)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
