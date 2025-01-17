// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test case to demonstrate setting worksheet zoom.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_zoom(200);

    let worksheet = workbook.add_worksheet();
    worksheet.set_zoom(75);

    // This zoom is the default and should be ignored.
    let worksheet = workbook.add_worksheet();
    worksheet.set_zoom(100);

    // These zooms are outside the allowed range and should be ignored.
    let worksheet = workbook.add_worksheet();
    worksheet.set_zoom(401);
    worksheet.set_zoom(9);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap40_zoom() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap40")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
