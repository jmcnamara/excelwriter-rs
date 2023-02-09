// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Chart, ChartSeries, Workbook, XlsxError};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let series1 = ChartSeries::new()
        .set_categories("Sheet1", 0, 0, 4, 0)
        .set_values("Sheet1", 0, 1, 4, 1);

    let series2 = ChartSeries::new()
        .set_categories("Sheet1", 0, 0, 4, 0)
        .set_values("Sheet1", 0, 2, 4, 2);

    let mut chart = Chart::new().add_series(&series1).add_series(&series2);

    chart.set_axis_ids(64446848, 64448384);

    worksheet.insert_chart(8, 4, &chart)?;

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let series1 = ChartSeries::new()
        .set_categories("Sheet2", 0, 0, 4, 0)
        .set_values("Sheet2", 0, 1, 4, 1);

    let series2 = ChartSeries::new()
        .set_categories("Sheet2", 0, 0, 4, 0)
        .set_values("Sheet2", 0, 2, 4, 2);

    let mut chart = Chart::new().add_series(&series1).add_series(&series2);

    chart.set_axis_ids(85389696, 85391232);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar04() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
