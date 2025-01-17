// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let top_left_bottom = Format::new()
        .set_border_left(FormatBorder::Thin)
        .set_border_top(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin);

    let top_bottom = Format::new()
        .set_border_top(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin);

    let top_left = Format::new()
        .set_border_left(FormatBorder::Thin)
        .set_border_top(FormatBorder::Thin);

    worksheet.write_string_with_format(1, 1, "test", &top_left_bottom)?;
    worksheet.write_string_with_format(1, 3, "test", &top_left)?;
    worksheet.write_string_with_format(1, 5, "test", &top_bottom)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format12() {
    let test_runner = common::TestRunner::new()
        .set_name("format12")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
