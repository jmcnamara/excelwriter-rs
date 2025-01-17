// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Color, Workbook, XlsxError};

// Test to demonstrate setting the worksheet tab color.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "Foo")?;
    worksheet.set_tab_color(Color::Red);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_tab_color01() {
    let test_runner = common::TestRunner::new()
        .set_name("tab_color01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
