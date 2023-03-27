// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! An example of formatting the line color in a chart element.

use rust_xlsxwriter::{Chart, ChartLine, ChartType, Workbook, XlsxColor, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 40)?;
    worksheet.write(2, 0, 50)?;
    worksheet.write(3, 0, 20)?;
    worksheet.write(4, 0, 10)?;
    worksheet.write(5, 0, 50)?;

    // Create a simple Column chart.
    let mut chart = Chart::new(ChartType::Line);

    // Add a data series with formatting.
    chart
        .add_series()
        .set_values("Sheet1!$A$1:$A$6")
        .format()
        .set_line(&ChartLine::new().set_color(XlsxColor::RGB(0xFF9900)));

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
