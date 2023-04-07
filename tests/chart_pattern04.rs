// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{
    Chart, ChartFormat, ChartPatternFill, ChartPatternFillType, ChartType, Workbook, XlsxColor,
    XlsxError,
};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    for row_num in 0..3 {
        for col_num in 0..8 {
            worksheet.write_number(row_num as u32, col_num as u16, 2)?;
        }
    }

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(110_902_272, 110_756_608);

    chart
        .add_series()
        .set_values("=Sheet1!$A$1:$A$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::Dotted20Percent)
                    .set_foreground_color(XlsxColor::RGB(0xC0_00_00))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$B$1:$B$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::Dotted70Percent)
                    .set_foreground_color(XlsxColor::RGB(0xFF_00_00))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$C$1:$C$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::DiagonalStripesDarkDownwards)
                    .set_foreground_color(XlsxColor::RGB(0xFF_C0_00))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$D$1:$D$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::VerticalStripesNarrow)
                    .set_foreground_color(XlsxColor::RGB(0xFF_FF_00))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$E$1:$E$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::HorizontalStripesAlternating)
                    .set_foreground_color(XlsxColor::RGB(0x92_D0_50))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$F$1:$F$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::DiagonalBrick)
                    .set_foreground_color(XlsxColor::RGB(0x00_B0_50))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$G$1:$G$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::DottedDiamond)
                    .set_foreground_color(XlsxColor::RGB(0x00_B0_F0))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    chart
        .add_series()
        .set_values("=Sheet1!$H$1:$H$3")
        .set_format(
            ChartFormat::new().set_pattern_fill(
                ChartPatternFill::new()
                    .set_pattern(ChartPatternFillType::SmallCheckerboard)
                    .set_foreground_color(XlsxColor::RGB(0x00_70_C0))
                    .set_background_color(XlsxColor::RGB(0xFF_FF_FF)),
            ),
        );

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_pattern04() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_pattern04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
