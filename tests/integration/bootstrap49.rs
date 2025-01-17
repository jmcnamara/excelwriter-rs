// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Url, Workbook, XlsxError};

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_hyperlink();

    worksheet.write_url(0, 0, "https://www.rust-lang.org/")?;
    worksheet.write_url_with_text(2, 0, "https://www.rust-lang.org/", "Rust")?;
    worksheet.write_url_with_format(4, 0, "https://www.rust-lang.org/", &format)?;

    workbook.save(filename)?;

    Ok(())
}

// Test with Url struct.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_hyperlink();

    worksheet.write_url(0, 0, Url::new("https://www.rust-lang.org/"))?;
    worksheet.write_url_with_text(2, 0, Url::new("https://www.rust-lang.org/"), "Rust")?;
    worksheet.write_url_with_format(4, 0, Url::new("https://www.rust-lang.org/"), &format)?;

    workbook.save(filename)?;

    Ok(())
}

// Test with Url struct and generics.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_hyperlink();

    worksheet.write(0, 0, Url::new("https://www.rust-lang.org/"))?;
    worksheet.write(
        2,
        0,
        Url::new(String::from("https://www.rust-lang.org/")).set_text("Rust"),
    )?;
    worksheet.write_with_format(4, 0, Url::new("https://www.rust-lang.org/"), &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap49_hyperlinks_1() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap49")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap49_hyperlinks_2() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap49")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap49_hyperlinks_3() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap49")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
