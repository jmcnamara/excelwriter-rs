// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Color, Format, FormatPattern, Table, TableColumn, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    let format1 = Format::new()
        .set_foreground_color(Color::Red)
        .set_background_color(Color::Yellow)
        .set_pattern(FormatPattern::DarkVertical);

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format1),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(1, 2, 5, 5, &table)?;

    worksheet.write(2, 2, "Foo")?;
    worksheet.write(2, 4, 2000)?;
    worksheet.write(2, 5, 4321)?;
    worksheet.write_with_format(2, 3, 1234, &format1)?;

    worksheet.write(3, 2, "Bar")?;
    worksheet.write(3, 4, 4000)?;
    worksheet.write(3, 5, 4320)?;
    worksheet.write_with_format(3, 3, 1256, &format1)?;

    worksheet.write(4, 2, "Baz")?;
    worksheet.write(4, 4, 3000)?;
    worksheet.write(4, 5, 4332)?;
    worksheet.write_with_format(4, 3, 2234, &format1)?;

    worksheet.write(5, 2, "Bop")?;
    worksheet.write(5, 4, 1000)?;
    worksheet.write(5, 5, 4333)?;
    worksheet.write_with_format(5, 3, 1324, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

// Test setting the cell format from the table.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    let format1 = Format::new()
        .set_foreground_color(Color::Red)
        .set_background_color(Color::Yellow)
        .set_pattern(FormatPattern::DarkVertical);

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format1),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(1, 2, 5, 5, &table)?;

    worksheet.write(2, 2, "Foo")?;
    worksheet.write(2, 4, 2000)?;
    worksheet.write(2, 5, 4321)?;
    worksheet.write_with_format(2, 3, 1234, &format1)?;

    worksheet.write(3, 2, "Bar")?;
    worksheet.write(3, 4, 4000)?;
    worksheet.write(3, 5, 4320)?;
    worksheet.write_with_format(3, 3, 1256, &format1)?;

    worksheet.write(4, 2, "Baz")?;
    worksheet.write(4, 4, 3000)?;
    worksheet.write(4, 5, 4332)?;
    worksheet.write_with_format(4, 3, 2234, &format1)?;

    worksheet.write(5, 2, "Bop")?;
    worksheet.write(5, 4, 1000)?;
    worksheet.write(5, 5, 4333)?;
    worksheet.write_with_format(5, 3, 1324, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table31_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table31")
        .set_function(create_new_xlsx_file_1)
        .unique("optimize1")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_optimize_table31_2() {
    let test_runner = common::TestRunner::new()
        .set_name("table31")
        .set_function(create_new_xlsx_file_2)
        .unique("optimize2")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
