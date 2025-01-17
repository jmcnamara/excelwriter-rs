// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartEmptyCells, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [2, 7, 3, 6, 2])?;
    worksheet.write_column(0, 1, [20, 25, 10, 10, 20])?;

    let mut chart1 = Chart::new(ChartType::Area);
    chart1.set_axis_ids(91755648, 91757952);

    chart1.add_series().set_values(("Sheet1", 0, 0, 4, 0));

    chart1.x_axis().set_position_between_ticks(true);
    chart1.show_empty_cells_as(ChartEmptyCells::Gaps);

    let mut chart2 = Chart::new(ChartType::Column);
    chart2.add_series().set_values(("Sheet1", 0, 1, 4, 1));

    chart1.combine(&chart2);

    worksheet.insert_chart(8, 4, &chart1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_combined06() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_combined06")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
