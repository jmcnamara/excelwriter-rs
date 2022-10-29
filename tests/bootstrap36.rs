// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use chrono::NaiveDate;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_num_format("dd/mm/yyyy;@");
    let format2 = Format::new().set_num_format("mm/dd/yyyy;@");
    let format3 = Format::new().set_num_format("yyyy/mm/dd;@");
    let format4 = Format::new().set_num_format("dddd\\ dd\\ mmmm\\ yyyy;@");
    let format5 = Format::new().set_num_format("[$-F800]dddd\\,\\ mmmm\\ dd\\,\\ yyyy");
    let format6 = Format::new().set_num_format("[$-F400]h:mm:ss\\ AM/PM");

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30)?;

    let datetime = NaiveDate::from_ymd(2023, 1, 25).and_hms(0, 0, 0);
    let date = datetime.date();

    let datetime2 = NaiveDate::from_ymd(2023, 1, 25).and_hms(18, 0, 0);
    let time = datetime2.time();

    worksheet.write_datetime(0, 0, datetime, &format1)?;
    worksheet.write_datetime(1, 0, datetime, &format2)?;
    worksheet.write_date(2, 0, date, &format3)?;
    worksheet.write_date(3, 0, date, &format4)?;
    worksheet.write_datetime(4, 0, datetime2, &format5)?;
    worksheet.write_time(5, 0, time, &format6)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap36_date_time() {
    let test_runner = common::TestRunner::new("bootstrap36").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
