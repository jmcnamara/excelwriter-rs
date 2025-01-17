// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Shape, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let textbox = Shape::textbox().set_text("This is some text");

    worksheet.insert_shape(8, 4, &textbox)?;

    let textbox = Shape::textbox().set_text("Some more text");

    worksheet.insert_shape(17, 7, &textbox)?;

    let worksheet = workbook.add_worksheet();

    let textbox = Shape::textbox().set_text("Hello");

    worksheet.insert_shape(3, 2, &textbox)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_textbox03() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
