// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate autofit.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet
        .write_formula(0, 0, "=9999+1")?
        .set_formula_result(0, 0, "10000");

    worksheet.autofit();

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofit11() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit11")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
