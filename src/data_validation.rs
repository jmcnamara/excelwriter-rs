// data_validation - A module to represent Excel data validations.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! # Working with Data Validation
//!
//! TODO

#![warn(missing_docs)]

mod tests;

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    static_regex, utility, ColNum, ExcelDateTime, Formula, IntoExcelDateTime, RowNum, XlsxError,
    COL_MAX, ROW_MAX,
};
use std::{borrow::Cow, fmt};

// -----------------------------------------------------------------------
// DataValidation
// -----------------------------------------------------------------------

/// The `DataValidation` struct represents a Cell conditional format.
///
/// TODO
///
#[derive(Clone)]
pub struct DataValidation {
    pub(crate) validation_type: Option<DataValidationType>,
    pub(crate) rule: Option<DataValidationRuleInternal>,
    pub(crate) ignore_blank: bool,
    pub(crate) show_input_message: bool,
    pub(crate) show_error_message: bool,
    pub(crate) input_title: String,
    pub(crate) error_title: String,
    pub(crate) input_message: String,
    pub(crate) error_message: String,
    pub(crate) error_style: DataValidationErrorStyle,
}

impl DataValidation {
    /// Create a new Cell conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> DataValidation {
        DataValidation {
            validation_type: None,
            rule: None,
            ignore_blank: true,
            show_input_message: true,
            show_error_message: true,
            input_title: String::new(),
            error_title: String::new(),
            input_message: String::new(),
            error_message: String::new(),
            error_style: DataValidationErrorStyle::Stop,
        }
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_any_value(mut self) -> DataValidation {
        self.rule = Some(DataValidationRuleInternal::EqualTo(String::new()));
        self.validation_type = Some(DataValidationType::Any);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_whole_number(mut self, rule: DataValidationRule<i32>) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::Whole);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_whole_number_from_cell(
        mut self,
        rule: DataValidationRule<DataValidationRange>,
    ) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::Whole);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_decimal_number(mut self, rule: DataValidationRule<f64>) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::Decimal);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_list_from_strings(mut self, list: &[impl AsRef<str>]) -> DataValidation {
        let joined_list = list
            .iter()
            .map(|s| s.as_ref().to_string().replace('"', "\"\""))
            .collect::<Vec<String>>()
            .join(",");

        let length = joined_list.chars().count();
        if length > 255 {
            eprintln!(
                "Validation list length '{length }' including commas is greater than Excel's limit of 255 characters: {joined_list}"
            );
            return self;
        }

        let joined_list = format!("\"{joined_list}\"");

        self.rule = Some(DataValidationRuleInternal::ListSource(joined_list));
        self.validation_type = Some(DataValidationType::List);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_date(
        mut self,
        rule: DataValidationRule<impl IntoExcelDateTime + IntoDataValidationValue>,
    ) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::Date);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_time(
        mut self,
        rule: DataValidationRule<impl IntoExcelDateTime + IntoDataValidationValue>,
    ) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::Time);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_text_length(mut self, rule: DataValidationRule<u32>) -> DataValidation {
        // Change from a generic rule to a concrete internal rule.
        let rule = rule.to_internal_rule();
        self.rule = Some(rule);
        self.validation_type = Some(DataValidationType::TextLength);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn allow_custom_formula(mut self, rule: Formula) -> DataValidation {
        let formula = rule.expand_formula(true).to_string();
        self.rule = Some(DataValidationRuleInternal::CustomFormula(formula));

        self.validation_type = Some(DataValidationType::Custom);
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn ignore_blank(mut self, enable: bool) -> DataValidation {
        self.ignore_blank = enable;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn show_input_message(mut self, enable: bool) -> DataValidation {
        self.show_input_message = enable;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn show_error_message(mut self, enable: bool) -> DataValidation {
        self.show_error_message = enable;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn set_input_title(mut self, text: impl Into<String>) -> DataValidation {
        let text = text.into();
        let length = text.chars().count();

        if length > 32 {
            eprintln!(
                "Validation title length '{length }' greater than Excel's limit of 32 characters."
            );
            return self;
        }

        self.input_title = text;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn set_input_message(mut self, text: impl Into<String>) -> DataValidation {
        let text = text.into();
        let length = text.chars().count();

        if length > 255 {
            eprintln!(
                "Validation message length '{length }' greater than Excel's limit of 255 characters."
            );
            return self;
        }

        self.input_message = text;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn set_error_title(mut self, text: impl Into<String>) -> DataValidation {
        let text = text.into();
        let length = text.chars().count();

        if length > 32 {
            eprintln!(
                "Validation title length '{length }' greater than Excel's limit of 32 characters."
            );
            return self;
        }

        self.error_title = text;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn set_error_message(mut self, text: impl Into<String>) -> DataValidation {
        let text = text.into();
        let length = text.chars().count();

        if length > 255 {
            eprintln!(
                "Validation message length '{length }' greater than Excel's limit of 255 characters."
            );
            return self;
        }

        self.error_message = text;
        self
    }

    /// Set the TODO
    ///
    /// TODO
    ///
    pub fn set_error_style(mut self, error_style: DataValidationErrorStyle) -> DataValidation {
        self.error_style = error_style;
        self
    }

    // Validate the data validation.
    pub(crate) fn validate(&mut self) -> Result<(), XlsxError> {
        let Some(validation_type) = &self.validation_type else {
            return Err(XlsxError::DataValidationError(
                "DataValidation type must be set".to_string(),
            ));
        };

        // TODO - remove
        if *validation_type == DataValidationType::Any {
            self.rule = Some(DataValidationRuleInternal::EqualTo(String::new()));
        }

        if self.rule.is_none() {
            return Err(XlsxError::DataValidationError(
                "DataValidation rule must be set".to_string(),
            ));
        }

        Ok(())
    }

    // The "Any" validation type should be ignored if it doesn't have any input
    // or error titles or messages. This is the same rule as Excel.
    pub(crate) fn is_invalid_any(&mut self) -> bool {
        let Some(validation_type) = &self.validation_type else {
            return false;
        };

        *validation_type == DataValidationType::Any
            && self.input_title.is_empty()
            && self.input_message.is_empty()
            && self.error_title.is_empty()
            && self.error_message.is_empty()
    }
}

// -----------------------------------------------------------------------
// DataValidationValue
// -----------------------------------------------------------------------

/// The `DataValidationValue` struct represents conditional format value
/// types. TODO
///
/// Excel supports various types when specifying values in a conditional format
/// such as numbers, strings, dates, times and cell references.
/// `DataValidationValue` is used to support a similar generic interface to
/// conditional format values. It supports:
///
/// - Numbers: Any Rust number that can convert [`Into`] [`f64`].
/// - Strings: Any Rust string type that can convert into String such as
///   [`&str`], [`String`], `&String` and `Cow<'_, str>`.
/// - Dates/times: [`ExcelDateTime`] values and if the `chrono` feature is
///   enabled [`chrono::NaiveDateTime`], [`chrono::NaiveDate`] and
///   [`chrono::NaiveTime`].
/// - Cell ranges: Use [`Formula`] in order to distinguish from strings. For
///   example `Formula::new(=A1)`.
///
/// [`chrono::NaiveDate`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
/// [`chrono::NaiveTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html
/// [`chrono::NaiveDateTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html
///
#[derive(Clone)]
pub struct DataValidationValue {
    pub(crate) value: String,
}

impl DataValidationValue {
    pub(crate) fn new_from_string(value: impl Into<String>) -> DataValidationValue {
        DataValidationValue {
            value: value.into(),
        }
    }
}

impl fmt::Display for DataValidationValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// From/Into traits for DataValidationValue.
macro_rules! data_validation_value_from_string {
    ($($t:ty)*) => ($(
        impl From<$t> for DataValidationValue {
            fn from(value: $t) -> DataValidationValue {
                DataValidationValue::new_from_string(value)
            }
        }
    )*)
}
data_validation_value_from_string!(&str &String String Cow<'_, str>);

macro_rules! data_validation_value_from_number {
    ($($t:ty)*) => ($(
        impl From<$t> for DataValidationValue {
            fn from(value: $t) -> DataValidationValue {
                DataValidationValue::new_from_string(value.to_string())
            }
        }
    )*)
}
data_validation_value_from_number!(u8 i8 u16 i16 u32 i32 f32 f64);

impl From<&DataValidationRange> for DataValidationValue {
    fn from(value: &DataValidationRange) -> DataValidationValue {
        DataValidationValue::new_from_string(value.to_string())
    }
}

impl From<Formula> for DataValidationValue {
    fn from(value: Formula) -> DataValidationValue {
        DataValidationValue::new_from_string(value.expand_formula(true))
    }
}

impl From<ExcelDateTime> for DataValidationValue {
    fn from(value: ExcelDateTime) -> DataValidationValue {
        let value = value.to_excel().to_string();
        DataValidationValue::new_from_string(value)
    }
}

impl From<&ExcelDateTime> for DataValidationValue {
    fn from(value: &ExcelDateTime) -> DataValidationValue {
        let value = value.to_excel().to_string();
        DataValidationValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<&NaiveDate> for DataValidationValue {
    fn from(value: &NaiveDate) -> DataValidationValue {
        let value = ExcelDateTime::chrono_date_to_excel(value).to_string();
        DataValidationValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<&NaiveDateTime> for DataValidationValue {
    fn from(value: &NaiveDateTime) -> DataValidationValue {
        let value = ExcelDateTime::chrono_datetime_to_excel(value).to_string();
        DataValidationValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<&NaiveTime> for DataValidationValue {
    fn from(value: &NaiveTime) -> DataValidationValue {
        let value = ExcelDateTime::chrono_time_to_excel(value).to_string();
        DataValidationValue::new_from_string(value)
    }
}

/// Trait to map rust types into an [`DataValidationValue`] value.
///
/// The `IntoDataValidationValue` trait is used to map Rust types like
/// strings, numbers, dates, times and formulas into a generic type that can be
/// used to replicate Excel data types used in Data Validation.
///
/// See [`DataValidationValue`] for more information.
///
pub trait IntoDataValidationValue {
    /// Function to turn types into a [`DataValidationValue`] enum value.
    fn new_value(&self) -> DataValidationValue;
}

impl IntoDataValidationValue for DataValidationValue {
    fn new_value(&self) -> DataValidationValue {
        self.clone()
    }
}

macro_rules! data_validation_value_from_type {
    ($($t:ty)*) => ($(
        impl IntoDataValidationValue for $t {
            fn new_value(&self) -> DataValidationValue {
                (*self).into()
            }
        }
    )*)
}

data_validation_value_from_type!(
    u8 i8 u16 i16 u32 i32 f32 f64
    &ExcelDateTime
);

impl IntoDataValidationValue for ExcelDateTime {
    fn new_value(&self) -> DataValidationValue {
        self.into()
    }
}

impl IntoDataValidationValue for DataValidationRange {
    fn new_value(&self) -> DataValidationValue {
        self.into()
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
data_validation_value_from_type!(&NaiveDate & NaiveDateTime & NaiveTime);

// -----------------------------------------------------------------------
// DataValidationRange
// -----------------------------------------------------------------------

/// TODO
pub struct DataValidationRange {
    first_row: RowNum,
    first_col: ColNum,
    last_row: RowNum,
    last_col: ColNum,
    range_string: String,
}

impl DataValidationRange {
    /// Create a new `DataValidationRange` from a worksheet cell 2 tuple.
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Examples
    ///
    /// The following example demonstrates creating a new data validation range.
    ///
    /// ```
    /// # // This code is available in examples/doc_DataValidationRange_new_from_range.rs TODO
    /// #
    /// # use rust_xlsxwriter::DataValidationRange;
    /// #
    /// # #[allow(unused_variables)]
    /// # fn main() {
    ///     // Same as "A5".
    ///     let range = DataValidationRange::new_from_cell(4, 0);
    /// # }
    /// ```
    ///
    pub fn new_from_cell(row: RowNum, col: ColNum) -> Result<DataValidationRange, XlsxError> {
        let mut range = DataValidationRange {
            first_row: row,
            first_col: col,
            last_row: row,
            last_col: col,
            range_string: String::new(),
        };

        Self::validate(&range)?;
        Self::cells_to_range(&mut range);

        Ok(range)
    }

    /// Create a new `DataValidationRange` from a worksheet cell 4 tuple range.
    ///
    /// # Errors
    ///
    /// TODO
    /// # Examples
    ///
    /// The following example demonstrates creating a new data validation range.
    ///
    /// ```
    /// # // This code is available in examples/doc_DataValidationRange_new_from_range.rs TODO
    /// #
    /// # use rust_xlsxwriter::DataValidationRange;
    /// #
    /// # #[allow(unused_variables)]
    /// # fn main() {
    ///     // Same as "A1:A5".
    ///     let range = DataValidationRange::new_from_range(0, 0, 4, 0);
    /// # }
    /// ```
    ///
    pub fn new_from_range(
        first_row: RowNum,
        first_col: ColNum,
        last_row: RowNum,
        last_col: ColNum,
    ) -> Result<DataValidationRange, XlsxError> {
        let mut range = DataValidationRange {
            first_row,
            first_col,
            last_row,
            last_col,
            range_string: String::new(),
        };

        Self::validate(&range)?;
        Self::cells_to_range(&mut range);

        Ok(range)
    }

    /// Create a new `DataValidationRange` from an Excel range formula.
    ///
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Examples
    ///
    /// The following example demonstrates creating a new chart range.
    ///
    ///
    /// ```
    /// # // This code is available in examples/doc_DataValidationRange_new_from_string.rs
    /// #
    /// # use rust_xlsxwriter::DataValidationRange;
    /// #
    /// # #[allow(unused_variables)]
    /// # fn main() {
    ///     let range = DataValidationRange::new_from_string("$A$1:$A$5");
    /// # }
    /// ```
    ///
    pub fn new_from_string(range_string: &str) -> Result<DataValidationRange, XlsxError> {
        let cell = static_regex!(r"^\$?([A-Z]+)\$?(\d+)$");
        let range = static_regex!(r"\$?([A-Z]+)\$?(\d+):\$?([A-Z]+)\$?(\d+)$");

        let first_row;
        let first_col;
        let last_row;
        let last_col;

        if let Some(caps) = range.captures(range_string) {
            first_row = caps.get(2).unwrap().as_str().parse::<u32>().unwrap() - 1;
            last_row = caps.get(4).unwrap().as_str().parse::<u32>().unwrap() - 1;
            first_col = utility::column_name_to_number(caps.get(1).unwrap().as_str());
            last_col = utility::column_name_to_number(caps.get(3).unwrap().as_str());
        } else if let Some(caps) = cell.captures(range_string) {
            first_row = caps.get(2).unwrap().as_str().parse::<u32>().unwrap() - 1;
            first_col = utility::column_name_to_number(caps.get(1).unwrap().as_str());
            last_row = first_row;
            last_col = first_col;
        } else {
            return Err(XlsxError::DataValidationError(format!(
                "Couldn't parse cell range '{range_string}'"
            )));
        }

        let range = DataValidationRange {
            first_row,
            first_col,
            last_row,
            last_col,
            range_string: range_string.to_string(),
        };

        Self::validate(&range)?;

        Ok(range)
    }

    // Convert the row/col into a range string.
    pub(crate) fn cells_to_range(&mut self) {
        let range1 = utility::row_col_to_cell(self.first_row, self.first_col);
        let range2 = utility::row_col_to_cell(self.last_row, self.last_col);

        if range1 == range2 {
            self.range_string = range1;
        } else {
            self.range_string = format!("{range1}:{range2}");
        }
    }

    // Convert the row/col into a range error string.
    pub(crate) fn error_range(&self) -> String {
        let last_row = self.last_row;
        let last_col = self.last_col;
        let first_row = self.first_row;
        let first_col = self.first_col;

        let range1 = utility::row_col_to_cell(self.first_row, self.first_col);
        let range2 = utility::row_col_to_cell(self.last_row, self.last_col);

        if range1 == range2 {
            format!("{range1}/({first_row}, {first_col})")
        } else {
            format!("{range1}:{range2}/({first_row}, {first_col}, {last_row}, {last_col})")
        }
    }

    // Check that the row/column values in the range are valid.
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        let range = self.error_range();

        if self.first_row > self.last_row {
            return Err(XlsxError::DataValidationError(format!(
                "Range '{range}' has a first row '{}' greater than the last row '{}'",
                self.first_row, self.last_row
            )));
        }

        if self.first_col > self.last_col {
            return Err(XlsxError::DataValidationError(format!(
                "Range '{range}' has a first column '{}' greater than the last column '{}'",
                self.first_col, self.last_col
            )));
        }

        if self.first_row >= ROW_MAX || self.last_row >= ROW_MAX {
            return Err(XlsxError::DataValidationError(format!(
                "Range '{range}' has a row '{}/{}' greater than Excel limit of 1048576",
                self.first_row, self.last_row
            )));
        }

        if self.first_col >= COL_MAX || self.last_col >= COL_MAX {
            return Err(XlsxError::DataValidationError(format!(
                "Range '{range}' has a column '{}/{}' greater than Excel limit of XFD/16384",
                self.first_col, self.last_col
            )));
        }

        Ok(())
    }
}

impl fmt::Display for DataValidationRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.range_string)
    }
}

// -----------------------------------------------------------------------
// DataValidationType
// -----------------------------------------------------------------------

/// The `DataValidationType` enum defines TODO
///
///
#[derive(Clone, Eq, PartialEq)]
pub enum DataValidationType {
    /// TODO
    Whole,

    /// TODO
    Decimal,

    /// TODO
    Date,

    /// TODO
    Time,

    /// TODO
    TextLength,

    /// TODO
    Custom,

    /// TODO
    List,

    /// TODO
    Any,
}

impl fmt::Display for DataValidationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Date => write!(f, "date"),
            Self::List => write!(f, "list"),
            Self::Time => write!(f, "time"),
            Self::Whole => write!(f, "whole"),
            Self::Custom => write!(f, "custom"),
            Self::Decimal => write!(f, "decimal"),
            Self::TextLength => write!(f, "textLength"),
        }
    }
}

// -----------------------------------------------------------------------
// DataValidationRule
// -----------------------------------------------------------------------

/// The `DataValidationRule` enum defines the conditional format rule for
/// [`DataValidation`].
///
///
#[derive(Clone)]
pub enum DataValidationRule<T: IntoDataValidationValue> {
    /// TODO.
    EqualTo(T),

    /// TODO.
    NotEqualTo(T),

    /// TODO.
    GreaterThan(T),

    /// TODO.
    GreaterThanOrEqualTo(T),

    /// TODO.
    LessThan(T),

    /// TODO.
    LessThanOrEqualTo(T),

    /// TODO.
    Between(T, T),

    /// TODO.
    NotBetween(T, T),
}

impl<T: IntoDataValidationValue> DataValidationRule<T> {
    fn to_internal_rule(&self) -> DataValidationRuleInternal {
        match &self {
            DataValidationRule::EqualTo(value) => {
                DataValidationRuleInternal::EqualTo(value.new_value().to_string())
            }
            DataValidationRule::NotEqualTo(value) => {
                DataValidationRuleInternal::NotEqualTo(value.new_value().to_string())
            }
            DataValidationRule::GreaterThan(value) => {
                DataValidationRuleInternal::GreaterThan(value.new_value().to_string())
            }

            DataValidationRule::GreaterThanOrEqualTo(value) => {
                DataValidationRuleInternal::GreaterThanOrEqualTo(value.new_value().to_string())
            }
            DataValidationRule::LessThan(value) => {
                DataValidationRuleInternal::LessThan(value.new_value().to_string())
            }
            DataValidationRule::LessThanOrEqualTo(value) => {
                DataValidationRuleInternal::LessThanOrEqualTo(value.new_value().to_string())
            }
            DataValidationRule::Between(min, max) => DataValidationRuleInternal::Between(
                min.new_value().to_string(),
                max.new_value().to_string(),
            ),
            DataValidationRule::NotBetween(min, max) => DataValidationRuleInternal::NotBetween(
                min.new_value().to_string(),
                max.new_value().to_string(),
            ),
        }
    }
}

// -----------------------------------------------------------------------
// DataValidationRuleInternal
// -----------------------------------------------------------------------

// TODO
#[derive(Clone)]
pub(crate) enum DataValidationRuleInternal {
    EqualTo(String),

    NotEqualTo(String),

    GreaterThan(String),

    GreaterThanOrEqualTo(String),

    LessThan(String),

    LessThanOrEqualTo(String),

    Between(String, String),

    NotBetween(String, String),

    CustomFormula(String),

    ListSource(String),
}

impl fmt::Display for DataValidationRuleInternal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EqualTo(_) => write!(f, "equal"),
            Self::LessThan(_) => write!(f, "lessThan"),
            Self::Between(_, _) => write!(f, "between"),
            Self::ListSource(_) => write!(f, "list"),
            Self::NotEqualTo(_) => write!(f, "notEqual"),
            Self::GreaterThan(_) => write!(f, "greaterThan"),
            Self::CustomFormula(_) => write!(f, ""),
            Self::NotBetween(_, _) => write!(f, "notBetween"),
            Self::LessThanOrEqualTo(_) => write!(f, "lessThanOrEqual"),
            Self::GreaterThanOrEqualTo(_) => write!(f, "greaterThanOrEqual"),
        }
    }
}

// -----------------------------------------------------------------------
// DataValidationErrorStyle
// -----------------------------------------------------------------------

/// The `DataValidationErrorStyle` enum defines TODO
///
///
#[derive(Clone)]
pub enum DataValidationErrorStyle {
    /// TODO
    Stop,

    /// TODO
    Warning,

    /// TODO
    Information,
}

impl fmt::Display for DataValidationErrorStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stop => write!(f, "stop"),
            Self::Warning => write!(f, "warning"),
            Self::Information => write!(f, "information"),
        }
    }
}
