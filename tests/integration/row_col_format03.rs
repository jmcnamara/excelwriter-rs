// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Test to demonstrate row or column formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let italic = Format::new().set_italic();

    worksheet.set_column_format(0, &italic)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_row_col_format03() {
    let test_runner = common::TestRunner::new()
        .set_name("row_col_format03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
