// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_default_row_height(24);

    worksheet.write(0, 0, "Foo")?;
    worksheet.write(9, 0, "Bar")?;

    workbook.save(filename)?;

    Ok(())
}

fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_default_row_height_pixels(32);

    worksheet.write(0, 0, "Foo")?;
    worksheet.write(9, 0, "Bar")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_default_row01_1() {
    let test_runner = common::TestRunner::new()
        .set_name("default_row01")
        .unique("1")
        .set_function(create_new_xlsx_file_1)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_default_row01_2() {
    let test_runner = common::TestRunner::new()
        .set_name("default_row01")
        .unique("2")
        .set_function(create_new_xlsx_file_2)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
