// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate setting margins.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_margins(1.0, 1.25, 1.5, 1.75, 0.75, 0.25);

    let worksheet = workbook.add_worksheet();
    worksheet.set_margins(-1.0, -1.0, -1.0, -1.0, -1.0, -1.0);

    let worksheet = workbook.add_worksheet();
    worksheet.set_margins(0.25, 0.25, 0.75, 0.75, 0.3, 0.3);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap42_margins() {
    let test_runner = common::TestRunner::new("bootstrap42").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
