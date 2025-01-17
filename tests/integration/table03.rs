// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width(2, 10.288)?;
    worksheet.set_column_width(3, 10.288)?;
    worksheet.set_column_width(4, 10.288)?;
    worksheet.set_column_width(5, 10.288)?;

    let table = Table::new();
    worksheet.add_table(2, 2, 12, 5, &table)?;

    worksheet.write_url(15, 0, "http://perl.com/")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table03() {
    let test_runner = common::TestRunner::new()
        .set_name("table03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
