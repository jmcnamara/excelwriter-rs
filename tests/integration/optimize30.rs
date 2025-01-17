// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    worksheet.write_array_formula(0, 0, 2, 0, "=SUM(B1:C1*B2:C2)")?;

    worksheet.write_number(0, 1, 0)?;
    worksheet.write_number(0, 2, 0)?;
    worksheet.write_number(1, 1, 0)?;
    worksheet.write_number(1, 2, 0)?;
    worksheet.write_number(2, 1, 0)?;
    worksheet.write_number(2, 2, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize30() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize30")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
