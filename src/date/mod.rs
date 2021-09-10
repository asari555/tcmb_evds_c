use std::str;

mod date_validation_basics;

use self::date_validation_basics::*;

use crate::traits::{self, HavingDateValidation};
use crate::error::ReturnError;


/// supplies single data series.
///
/// Users need to create Date variable via [`from`](fn@crate::date::Date::from) to be sure given date fulfilling 
/// the requirements.
///
/// Date formats given by users and the template ("day-month-year", e.g. "01-01-2021") that have to be the same.
pub(crate) struct Date(String);

impl Date {
    /// creates a Date object from given valid date format.
    ///
    /// Valid date format is "day-month-year". For example, "01-01-2021".
    ///
    /// # Error
    ///
    /// Nonexistent days, months, years and improper date format cause error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # use tcmb_evds_c::error::ReturnError;
    ///     use tcmb_evds_c::date::Date;
    ///
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {     
    /// #
    ///     let date = Date::from("13-12-2011")?;
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) fn from(single_date: &str) -> Result<Date, ReturnError> {
        let date = Date(single_date.to_string());

        if date.is_given_date_valid() {
            return Ok(date)
        }
        
        Err(ReturnError::InvalidDate)
    }

    fn get_date(&self) -> &str {
        &self.0
    }
}

impl HavingDateValidation for Date {
    fn is_given_date_empty(&self) -> bool {
        if self.0.is_empty() { return true }

        false
    }

    fn is_string_length_valid(&self) -> bool {
        let valid_date_length = 10;

        if self.0.len() != valid_date_length { return false }

        true
    }

    fn is_given_date_valid(&self) -> bool {        
        if self.is_given_date_empty() { return false }
        if !self.is_string_length_valid() { return false }
        
        if !is_each_value_valid(&self.0) { return false; }
    
        if !is_alignment_valid(&self.0) { return false; }
       
        true
    }
}


/// supplies date range to the functions of web service operations requiring the range.
/// 
/// Date formats and the template ("day-month-year", exp. "01-01-2021") have to be the same.

pub(crate) struct DateRange {
    start_date: String,
    end_date: String,
}

impl DateRange {
    /// creates date range with default valid values.
    ///
    /// Default start and end dates: 13-12-2011 and 13-12-2012.
    pub(crate) fn new() -> DateRange {
        DateRange {
            start_date: "13-12-2011".to_string(),
            end_date: "13-12-2012".to_string(),
        }
    }

    /// creates customized date range.
    /// 
    /// # Error
    ///
    /// Invalid date formats (valid date format: 12-09-2019, day-month-year) cause returning error.
    /// 
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// # use tcmb_evds_c::error::ReturnError;
    ///     use tcmb_evds_c::date::DateRange;
    ///
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {     
    /// #
    ///     let date_range = DateRange::from("13-12-2011", "13-12-2020")?;
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) fn from(start_date: &str, end_date: &str) -> Result<DateRange, ReturnError> {

        let date_range = DateRange {
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        };

        if date_range.is_given_date_valid() {
            return Ok(date_range);
        }

        Err(ReturnError::InvalidDate)
    }

    /// modifies start date.
    pub(crate) fn modify_start(&mut self, start_date: &str) -> Result<(), ReturnError> {
        
        Date::from(start_date)?;

        self.start_date = start_date.to_string();

        Ok(())
    }

    /// modifies end date.
    pub(crate) fn modify_end(&mut self, end_date: &str) -> Result<(), ReturnError>  {

        Date::from(end_date)?;

        self.end_date = end_date.to_string();

        Ok(())
    }

    /// check emptiness of dates of the created date range.
    pub(crate) fn is_empty(&self) -> bool {
        if !self.start_date.is_empty() { return false }
        if !self.end_date.is_empty() { return false }

        true
    }

    /// gives dates in a tuple format.
    fn get_dates(&self) -> (&str, &str) {
        (&self.start_date, &self.end_date)
    }
}

impl HavingDateValidation for DateRange {
    fn is_given_date_empty(&self) -> bool {
        if !self.start_date.is_empty() { return false }
        if !self.end_date.is_empty() { return false }

        true
    }

    /// checks the lengths of date formats that equal wether valid date format length or not.
    fn is_string_length_valid(&self) -> bool {
        let valid_date_format_length: usize = 10;

        if self.start_date.len() != valid_date_format_length { return false }
        if self.end_date.len() != valid_date_format_length { return false }
        
        true
    }

    /// checks emptiness, length and alignment of the given dates. 
    fn is_given_date_valid(&self) -> bool {
        if self.is_empty() { return false }
        if !self.is_string_length_valid() { return false }

        if !is_each_value_valid(&self.start_date) { return false; }
        if !is_each_value_valid(&self.end_date) { return false; }

        if !is_alignment_valid(&self.start_date) { return false; }
        if !is_alignment_valid(&self.end_date) { return false; }

        true
    }
}


/// is the preference interface of [`Date`](crate::date::Date) and [`DateRange`](crate::date::DateRange) for 
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency) functions.
/// 
/// Some functions of [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency) requires the 
/// *DatePreference* to differentiate between [`Date`](crate::date::Date) and [`DateRange`](crate::date::DateRange). 
///
/// Note: *Single* date which is implemented as giving the same date to start and end dates.
pub(crate) enum DatePreference {
    Single(Date),
    Multiple(DateRange),
}

impl DatePreference {
    /// checks date preference is wether single or not.
    pub(crate) fn is_single(&self) -> Result<(), ReturnError> {
        if let DatePreference::Multiple(_) = self {
            return Err(ReturnError::SingleDateExpected);
        }

        Ok(())
    }

    /// checks date preference is wether single or not.
    pub(crate) fn is_multiple(&self) -> Result<(), ReturnError> {
        if let DatePreference::Single(_) = self {
            return Err(ReturnError::MultipleDateExpected);
        }

        Ok(())
    }

    /// gives the same dates in a tuple if date preference is *Single* or 
    /// start and end dates if date preference is *Multiple*.
    pub(crate) fn get_dates(&self) -> (&str, &str) {
        match self {
            DatePreference::Single(date) => {
                (date.get_date(), date.get_date())
            },
            DatePreference::Multiple(dates) => {
                dates.get_dates()
            },
        }
    }
}

impl traits::MakingUrlFormat for DatePreference {
    fn generate_url_format(&self) -> String {
        match &self {
            &Self::Single(date) => {
                format!("startDate={}&endDate={}", &date.0, &date.0)
            },
            &Self::Multiple(date_range) => {
                format!("startDate={}&endDate={}", &date_range.start_date, &date_range.end_date)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_do() {
        //
        let date_result = Date::from("12-11-2010");

        let date = date_result.unwrap();

        assert_eq!("12-11-2010", date.get_date());

        let date_result = Date::from("1123-2020");

        if let Err(return_error) = date_result {
            if return_error != ReturnError::InvalidDate { 
                panic!("Expected {}, got {}", ReturnError::InvalidDate.to_string(), return_error.to_string()); 
            }
        }

        assert!(!date.is_given_date_empty());
        assert!(date.is_string_length_valid());
        assert!(date.is_given_date_valid());

        assert!(is_each_value_valid("12-11-2012"));


        //
        let date_result = Date::from("12-11-2010");

        let date = date_result.unwrap();

        assert_eq!("12-11-2010", date.get_date());

        let date_result = Date::from("12-11-");

        if let Err(return_error) = date_result {
            if return_error != ReturnError::InvalidDate { 
                panic!("Expected {}, got {}", ReturnError::InvalidDate.to_string(), return_error.to_string()); 
            }
        }

        assert!(!date.is_given_date_empty());
        assert!(date.is_string_length_valid());
        assert!(date.is_given_date_valid());

        assert!(is_each_value_valid("12-11-2012"));

        
        //
        let date_range_result = DateRange::from("12-11-2015", "12-11-2016");

        let mut date_range = date_range_result.unwrap();

        assert!(date_range.is_given_date_valid());
        assert!(!date_range.is_given_date_empty());
        assert!(!date_range.is_empty());
        assert!(date_range.is_string_length_valid());

        let end_date_result = date_range.modify_end("11-12-2013");
        let start_date_result = date_range.modify_start("12-12-2012");

        if let Err(return_error) = end_date_result {
            println!("end date result returned {}", return_error);
        }
        if let Err(return_error) = start_date_result {
            println!("end date result returned {}", return_error);
        }
    }
}
