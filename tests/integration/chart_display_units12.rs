// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartAxisDisplayUnitType, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [10000000, 20000000, 30000000, 20000000, 10000000];
    worksheet.write_column(0, 0, data)?;
    worksheet.write_column(0, 1, data)?;

    let mut chart = Chart::new(ChartType::Scatter);
    chart.set_axis_ids(93550464, 93548544);
    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 1, 4, 1));

    chart
        .y_axis()
        .set_display_unit_type(ChartAxisDisplayUnitType::Hundreds)
        .set_display_units_visible(false);

    chart
        .x_axis()
        .set_display_unit_type(ChartAxisDisplayUnitType::Thousands)
        .set_display_units_visible(false);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_display_units12() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_display_units12")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
