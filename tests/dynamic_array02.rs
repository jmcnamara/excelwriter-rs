// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to test dynamic array formula: with explicit prefix.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_formula_only(0, 1, "=_xlfn.UNIQUE(A1)")?;
    worksheet.write_number_only(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with implicit prefix.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_formula_only(0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number_only(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with standard formula function.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_formula_only(0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number_only(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with standard array formula
// function.
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_array_formula_only(0, 1, 0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number_only(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array02_1() {
    let test_runner = common::TestRunner::new("dynamic_array02")
        .unique("1")
        .initialize();

    _ = create_new_xlsx_file_1(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_2() {
    let test_runner = common::TestRunner::new("dynamic_array02")
        .unique("2")
        .initialize();

    _ = create_new_xlsx_file_2(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_3() {
    let test_runner = common::TestRunner::new("dynamic_array02")
        .unique("3")
        .initialize();

    _ = create_new_xlsx_file_3(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_4() {
    let test_runner = common::TestRunner::new("dynamic_array02")
        .unique("4")
        .initialize();

    _ = create_new_xlsx_file_4(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
