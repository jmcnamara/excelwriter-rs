// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::default();

    worksheet.write_url_with_options(
        0,
        0,
        "http://example.com/%5b0%5d",
        "http://example.com/[0]",
        "",
        Some(&format),
    )?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_escapes08() {
    let test_runner = common::TestRunner::new()
        .set_name("escapes08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
