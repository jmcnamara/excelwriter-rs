// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! Example of adding a data validation to a worksheet cell. This validation
//! restricts input to integer values based on a value from another cell.

use rust_xlsxwriter::{DataValidation, DataValidationRule, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Upper limit:")?;
    worksheet.write(0, 2, 10)?;
    worksheet.write(1, 0, "Enter value in cell C2:")?;

    let data_validation = DataValidation::new()
        .allow_whole_number_formula(DataValidationRule::LessThanOrEqualTo("=C1".into()));

    worksheet.add_data_validation(1, 2, 1, 2, &data_validation)?;

    // Save the file.
    workbook.save("data_validation.xlsx")?;

    Ok(())
}
