// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format = Format::new().set_align(FormatAlign::Center);

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.write(0, 5, 123)?;
    worksheet.merge_range(1, 1, 5, 3, "", &format)?;

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.write(1, 5, 123)?;
    worksheet.merge_range(1, 1, 5, 3, "", &format)?;

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.merge_range(1, 1, 5, 3, "", &format)?;
    worksheet.write(3, 5, 123)?;

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.merge_range(1, 1, 5, 3, "", &format)?;
    worksheet.write(5, 5, 123)?;

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.merge_range(1, 1, 5, 3, "", &format)?;
    worksheet.write(6, 5, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize28() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize28")
        .set_function(create_new_xlsx_file)
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
