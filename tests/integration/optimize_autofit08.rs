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

    worksheet.write_string(0, 0, "a")?;
    worksheet.autofit();
    worksheet.write_string(1, 0, "aaa")?;
    worksheet.autofit();
    worksheet.write_string(2, 0, "a")?;
    worksheet.autofit();
    worksheet.write_string(3, 0, "aaaa")?;
    worksheet.autofit();
    worksheet.write_string(4, 0, "a")?;
    worksheet.autofit();

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_autofit08() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit08")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
