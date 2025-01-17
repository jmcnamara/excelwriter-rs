// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate print-across print option.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_page_order(false);
    worksheet.set_paper_size(9);
    worksheet.set_portrait(); // Secondary test. Should be the default.

    worksheet.write_string(0, 0, "Foo")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_print_across01() {
    let test_runner = common::TestRunner::new()
        .set_name("print_across01")
        .set_function(create_new_xlsx_file)
        .ignore_elements("xl/worksheets/sheet1.xml", "<pageMargins")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
