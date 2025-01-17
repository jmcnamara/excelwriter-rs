// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate worksheet cell selection.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet1 = workbook.add_worksheet();
    worksheet1.set_selection(3, 2, 3, 2)?;

    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_selection(3, 2, 6, 6)?;

    let worksheet3 = workbook.add_worksheet();
    worksheet3.set_selection(6, 6, 3, 2)?;

    let worksheet4 = workbook.add_worksheet();
    worksheet4.set_selection(3, 2, 3, 2)?;

    let worksheet5 = workbook.add_worksheet();
    worksheet5.set_selection(3, 2, 6, 6)?;

    let worksheet6 = workbook.add_worksheet();
    worksheet6.set_selection(6, 6, 3, 2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_selection02() {
    let test_runner = common::TestRunner::new()
        .set_name("selection02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
