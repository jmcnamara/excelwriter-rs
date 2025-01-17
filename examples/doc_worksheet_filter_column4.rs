// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates setting an autofilter with different list
//! filter conditions in separate columns.

use rust_xlsxwriter::{FilterCondition, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet with some sample data to filter.
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Region")?;
    worksheet.write_string(1, 0, "East")?;
    worksheet.write_string(2, 0, "West")?;
    worksheet.write_string(3, 0, "East")?;
    worksheet.write_string(4, 0, "North")?;
    worksheet.write_string(5, 0, "South")?;
    worksheet.write_string(6, 0, "West")?;

    worksheet.write_string(0, 1, "Sales")?;
    worksheet.write_number(1, 1, 3000)?;
    worksheet.write_number(2, 1, 8000)?;
    worksheet.write_number(3, 1, 5000)?;
    worksheet.write_number(4, 1, 4000)?;
    worksheet.write_number(5, 1, 7000)?;
    worksheet.write_number(6, 1, 9000)?;

    // Set the autofilter.
    worksheet.autofilter(0, 0, 6, 1)?;

    // Set a filter condition for 2 separate columns.
    let filter_condition1 = FilterCondition::new().add_list_filter("East");
    worksheet.filter_column(0, &filter_condition1)?;

    let filter_condition2 = FilterCondition::new().add_list_filter(3000);
    worksheet.filter_column(1, &filter_condition2)?;

    workbook.save("worksheet.xlsx")?;

    Ok(())
}
