// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [27, 33, 44, 12, 1])?;
    worksheet.write_column(0, 1, [6, 8, 6, 4, 2])?;

    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(63591168, 63592704);
    chart.set_axis2_ids(65934464, 72628864);

    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));

    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_y2_axis(true);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar24() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar24")
        .ignore_elements("xl/workbook.xml", "<fileVersion")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
