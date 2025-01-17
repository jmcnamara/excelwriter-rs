// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_bold().set_foreground_color(Color::Red);

    let format2 = format1.clone().set_italic();

    worksheet.write_string_with_format(0, 0, "Foo", &format1)?;
    worksheet.write_string_with_format(1, 0, "Bar", &format2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format03() {
    let test_runner = common::TestRunner::new()
        .set_name("format03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
