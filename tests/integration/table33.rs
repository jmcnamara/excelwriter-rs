// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Formula, Table, TableColumn, TableFunction, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_num_format_index(2);

    worksheet.set_column_width(1, 10.288)?;
    worksheet.set_column_width(2, 10.288)?;
    worksheet.set_column_width(3, 10.288)?;
    worksheet.set_column_width(4, 10.288)?;
    worksheet.set_column_width(5, 10.288)?;
    worksheet.set_column_width(6, 10.288)?;
    worksheet.set_column_width(7, 10.288)?;
    worksheet.set_column_width(8, 10.288)?;
    worksheet.set_column_width(9, 10.288)?;
    worksheet.set_column_width(10, 10.288)?;

    worksheet.write(0, 0, "Column1")?;
    worksheet.write(0, 1, "Column2")?;
    worksheet.write(0, 2, "Column3")?;
    worksheet.write(0, 3, "Column4")?;
    worksheet.write(0, 4, "Column5")?;
    worksheet.write(0, 5, "Column6")?;
    worksheet.write(0, 6, "Column7")?;
    worksheet.write(0, 7, "Column8")?;
    worksheet.write(0, 8, "Column9")?;
    worksheet.write(0, 9, "Column10")?;
    worksheet.write(0, 10, "Total")?;

    worksheet.write(3, 1, 0)?;
    worksheet.write(3, 2, 0)?;
    worksheet.write(3, 3, 0)?;
    worksheet.write(3, 6, 0)?;
    worksheet.write(3, 7, 0)?;
    worksheet.write(3, 8, 0)?;
    worksheet.write(3, 9, 0)?;
    worksheet.write(3, 10, 0)?;
    worksheet.write(4, 1, 0)?;
    worksheet.write(4, 2, 0)?;
    worksheet.write(4, 3, 0)?;
    worksheet.write(4, 6, 0)?;
    worksheet.write(4, 7, 0)?;
    worksheet.write(4, 8, 0)?;
    worksheet.write(4, 9, 0)?;
    worksheet.write(4, 10, 0)?;

    let columns = vec![
        TableColumn::new().set_total_label("Total"),
        TableColumn::default(),
        TableColumn::new().set_total_function(TableFunction::Average),
        TableColumn::new().set_total_function(TableFunction::Count),
        TableColumn::new().set_total_function(TableFunction::CountNumbers),
        TableColumn::new().set_total_function(TableFunction::Max),
        TableColumn::new().set_total_function(TableFunction::Min),
        TableColumn::new().set_total_function(TableFunction::Sum),
        TableColumn::new().set_total_function(TableFunction::StdDev),
        TableColumn::new()
            .set_total_function(TableFunction::Custom(Formula::new("=SUM([Column10])")))
            .set_formula("SUM(Table1[@[Column1]:[Column3]])")
            .set_format(format),
    ];

    let table = Table::new().set_columns(&columns).set_total_row(true);

    worksheet.add_table(2, 1, 5, 10, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table33() {
    let test_runner = common::TestRunner::new()
        .set_name("table33")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
