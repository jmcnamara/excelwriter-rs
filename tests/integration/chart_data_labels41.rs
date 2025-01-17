// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    Chart, ChartBorder, ChartDataLabel, ChartFont, ChartType, Workbook, XlsxError,
};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [
        [1, 2, 3, 4, 5],
        [2, 4, 6, 8, 10],
        [3, 6, 9, 12, 15],
        [10, 20, 30, 40, 50],
    ];
    for (col_num, col_data) in data.iter().enumerate() {
        for (row_num, row_data) in col_data.iter().enumerate() {
            worksheet.write(row_num as u32, col_num as u16, *row_data)?;
        }
    }

    let data_labels = [ChartDataLabel::new()
        .set_value("=Sheet1!$D$1")
        .set_format(ChartBorder::new().set_color("#FF0000"))
        .set_font(&ChartFont::new())
        .to_custom()];

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(74893568, 80048128);
    chart
        .add_series()
        .set_values(("Sheet1", 0, 0, 4, 0))
        .set_data_label(ChartDataLabel::new().show_value())
        .set_custom_data_labels(&data_labels);

    chart.add_series().set_values(("Sheet1", 0, 1, 4, 1));
    chart.add_series().set_values(("Sheet1", 0, 2, 4, 2));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_data_labels41() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_data_labels41")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
