// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! Example of adding a data validation to a worksheet cell. This validation
//! restricts input to time values in a fixed range.

use rust_xlsxwriter::{DataValidation, DataValidationRule, ExcelDateTime, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(1, 0, "Enter value in cell C2:")?;

    let data_validation = DataValidation::new().allow_time(DataValidationRule::Between(
        ExcelDateTime::parse_from_str("6:00")?,
        ExcelDateTime::parse_from_str("12:00")?,
    ));

    worksheet.add_data_validation(1, 2, 1, 2, &data_validation)?;

    // Save the file.
    workbook.save("data_validation.xlsx")?;

    Ok(())
}
