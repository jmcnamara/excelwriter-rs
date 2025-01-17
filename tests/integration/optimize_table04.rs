// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Note, Table, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Pre-populate the string table to get the same order as Excel.
    workbook.populate_string_table("Column1", 0);
    workbook.populate_string_table("Column2", 1);
    workbook.populate_string_table("Column3", 2);
    workbook.populate_string_table("Column4", 3);

    let worksheet = workbook.add_worksheet_with_low_memory();

    // The following works around the <dimension> difference in constant memory.
    worksheet.write_number(0, 9, 1)?;
    worksheet.clear_cell(0, 9);

    worksheet.write_url_with_format(0, 0, "http://perl.com/", &Format::default())?;

    worksheet.set_column_width(2, 10.288)?;
    worksheet.set_column_width(3, 10.288)?;
    worksheet.set_column_width(4, 10.288)?;
    worksheet.set_column_width(5, 10.288)?;

    let table = Table::new();
    worksheet.add_table(2, 2, 12, 5, &table)?;

    worksheet.set_default_note_author("John");
    let note = Note::new("Test1").add_author_prefix(false);
    worksheet.insert_note(0, 7, &note)?;

    let note = Note::new("Test2").add_author_prefix(false);
    worksheet.insert_note(0, 9, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table04() {
    let test_runner = common::TestRunner::new()
        .set_name("table04")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
