// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate handling NaN and Inf numbers. For now these are
// handled as strings but at a latter stage them may be stored as Excel error
// types.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_number_only(0, 0, f64::NAN)?;
    worksheet.write_number_only(1, 0, f64::INFINITY)?;
    worksheet.write_number_only(2, 0, f64::NEG_INFINITY)?;

    worksheet.write_string_only(1, 0, "#DIV/0!")?;
    worksheet.write_string_only(2, 0, "#DIV/0!")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap16_test_nan() {
    let test_runner = common::TestRunner::new("bootstrap16").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
