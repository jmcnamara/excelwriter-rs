// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 11)?;
    worksheet.set_column_width(1, 11)?;
    worksheet.set_column_width(2, 11)?;
    worksheet.set_column_width(3, 11)?;

    // Add some test data for the chart(s).
    let headers = ["Series 1", "Series 2", "Series 3"];
    let categories = ["Category 1", "Category 2", "Category 3", "Category 4"];
    let numbers = [
        [4.3, 2.5, 3.5, 4.5],
        [2.4, 4.5, 1.8, 2.8],
        [2.0, 2.0, 3.0, 5.0],
    ];

    worksheet.write_row(0, 1, headers)?;
    worksheet.write_column(1, 0, categories)?;
    worksheet.write_column(1, 1, numbers[0])?;
    worksheet.write_column(1, 2, numbers[1])?;
    worksheet.write_column(1, 3, numbers[2])?;

    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(43706240, 43727104);
    chart
        .add_series()
        .set_categories(("Sheet1", 1, 0, 4, 0))
        .set_values(("Sheet1", 1, 1, 4, 1));

    chart
        .add_series()
        .set_categories(("Sheet1", 1, 0, 4, 0))
        .set_values(("Sheet1", 1, 2, 4, 2));

    chart
        .add_series()
        .set_categories(("Sheet1", 1, 0, 4, 0))
        .set_values(("Sheet1", 1, 3, 4, 3));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar23() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar23")
        .ignore_elements("xl/charts/chart1.xml", "sourceLinked")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
