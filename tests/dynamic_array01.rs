// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to test dynamic arrays/formulas.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_array_formula_only(0, 0, 0, 0, "=AVERAGE(TIMEVALUE(B1:B2))")?;
    worksheet.write_string_only(0, 1, "12:00")?;
    worksheet.write_string_only(1, 1, "12:00")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array01() {
    let test_runner = common::TestRunner::new("dynamic_array01").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
