// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate setting various page setup methods.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_first_page_number(1);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_scale(200);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_fit_to_pages(1, 1);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_fit_to_pages(2, 2);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_fit_to_pages(1, 0);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_center_horizontally(true);
    worksheet.set_print_center_vertically(true);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_print_headings(true);
    worksheet.set_print_gridlines(true);
    worksheet.set_print_black_and_white(true);
    worksheet.set_print_draft(true);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;
    worksheet.set_header_footer_align_with_page(false);
    worksheet.set_header_footer_scale_with_doc(false);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap43_page_setup() {
    let test_runner = common::TestRunner::new("bootstrap43").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
