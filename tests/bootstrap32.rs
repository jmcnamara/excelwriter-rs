// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxBorder, XlsxColor, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with cell borders.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_border(XlsxBorder::Thin);

    let format2 = Format::new()
        .set_border(XlsxBorder::Hair)
        .set_border_color(XlsxColor::Red);

    let format3 = Format::new()
        .set_border_top(XlsxBorder::Dotted)
        .set_border_left(XlsxBorder::DashDotDot)
        .set_border_right(XlsxBorder::Dashed)
        .set_border_bottom(XlsxBorder::DashDot)
        .set_border_top_color(XlsxColor::Red)
        .set_border_left_color(XlsxColor::Red)
        .set_border_right_color(XlsxColor::Red)
        .set_border_bottom_color(XlsxColor::Red);

    let format4 = Format::new()
        .set_border_top(XlsxBorder::MediumDashDotDot)
        .set_border_left(XlsxBorder::MediumDashDotDot)
        .set_border_right(XlsxBorder::MediumDashDot)
        .set_border_bottom(XlsxBorder::SlantDashDot)
        .set_border_top_color(XlsxColor::Red)
        .set_border_left_color(XlsxColor::Red)
        .set_border_right_color(XlsxColor::Red)
        .set_border_bottom_color(XlsxColor::Red);

    let format5 = Format::new()
        .set_border_top(XlsxBorder::MediumDashed)
        .set_border_left(XlsxBorder::Medium)
        .set_border_right(XlsxBorder::Double)
        .set_border_bottom(XlsxBorder::Thick)
        .set_border_top_color(XlsxColor::Red)
        .set_border_left_color(XlsxColor::Red)
        .set_border_right_color(XlsxColor::Red)
        .set_border_bottom_color(XlsxColor::Red);

    let worksheet = workbook.add_worksheet();

    worksheet.write_blank(2, 1, &format1)?;
    worksheet.write_blank(4, 1, &format2)?;
    worksheet.write_blank(6, 1, &format3)?;
    worksheet.write_blank(8, 1, &format4)?;
    worksheet.write_blank(10, 1, &format5)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap32_borders() {
    let test_runner = common::TestRunner::new("bootstrap32").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
