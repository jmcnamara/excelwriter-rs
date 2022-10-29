// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let wrap = Format::new().set_text_wrap();

    worksheet.set_row_height(0, 45)?;

    worksheet.write_string(0, 0, "Foo\nBar", &wrap)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format05() {
    let test_runner = common::TestRunner::new("format05").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
