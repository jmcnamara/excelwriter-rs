// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();

    // Constant memory worksheet.
    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.write_number_with_format(0, 0, 123, &bold)?;

    // Constant memory worksheet.
    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.write_number_with_format(0, 0, 123, &italic)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize22() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize22")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
