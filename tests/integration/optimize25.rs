// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2422-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let bold = Format::new().set_bold();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    worksheet.write_number_with_format(0, 0, 123, &bold)?;
    worksheet.write_url(1, 0, "https://www.rust-lang.org/")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize25() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize25")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
