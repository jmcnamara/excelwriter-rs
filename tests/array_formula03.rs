// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to test array formulas, single cell range.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_array_formula_only(0, 0, 0, 0, "{=SUM(B1:C1*B2:C2)}")?;

    worksheet.write_number_only(0, 1, 0)?;
    worksheet.write_number_only(1, 1, 0)?;
    worksheet.write_number_only(2, 1, 0)?;
    worksheet.write_number_only(0, 2, 0)?;
    worksheet.write_number_only(1, 2, 0)?;
    worksheet.write_number_only(2, 2, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_array_formula03() {
    let test_runner = common::TestRunner::new("array_formula03")
        .ignore_calc_chain()
        .initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
