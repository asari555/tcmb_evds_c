use libc::c_uint;

/// provides required enums for advanced currency operations.
///
/// These enums as arguments are used in [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data).
///
/// # Example
///
/// ```C
///     // frequency formulas declarations.
///     TcmbEvdsAggregationType aggregation_type;
///     TcmbEvdsFormula formula;
///     TcmbEvdsDataFrequency data_frequency;
///     bool ascii_mode;
///
///
///     // frequency formulas value assignments.
///     aggregation_type = End;
///     formula = Level;
///     data_frequency = Monthly;
///
///     ascii_mode = true; 
/// 
///
///     // frequency formulas are only used with tcmb_evds_c_get_advanced_data function.
///     TcmbEvdsResult advanced_data_result = 
///         tcmb_evds_c_get_advanced_data(
///             data_series, 
///             date, 
///             aggregation_type, 
///             formula, 
///             data_frequency, 
///             api_key, 
///             return_format,
///             ascii_mode,
///         );
/// ```
pub mod advanced_entities;
/// includes structures, enums and a function to make common operations for all FFI functions.
///
/// `TcmbEvdsInput` and `TcmbEvdsResult` are the structures to share strings with C functions and error information to 
/// handle.
///
/// # Example
///
/// ```C
///     // declaration of inputs.
///     TcmbEvdsInput data_series;
///     TcmbEvdsInput date;
///     TcmbEvdsInput api_key;
///     // return format declaration and assignment.
///     TcmbEvdsReturnFormat return_format = Csv;
///     bool ascii_mode;
///
///     // assignments of inputs.
///     data_series.input_ptr = "TP.DK.USD.S";
///     data_series.string_capacity = strlen(data_series.input_ptr);
/// 
///     date.input_ptr = "13-12-2011";
///     date.string_capacity = strlen(date.input_ptr);
/// 
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
/// 
///     ascii_mode = false;
/// 
/// 
///     // requesting data part.
///     TcmbEvdsResult data_result = tcmb_evds_c_get_data(data_series, date, api_key, return_format, ascii_mode);
/// 
///
///     // error checking and printing the result of the response.
///     if (!tcmb_evds_c_is_error(data_result)) { printf("\nNO ERROR!\n"); };
/// 
///     fwrite(data_result.output_ptr, data_result.string_capacity, 1, stdout);
///     fflush(stdout);
/// 
///     printf("\nError: %s", tcmb_evds_c_is_error(data_result) ? "true" : "false");
/// 
/// 
///     // result pointer must be freed.
///     free(data_result.output_ptr); 
/// ```
pub mod common_entities;
/// provides an error enum interface to easily handle the returned error.
///
/// # Example
///
/// ```C
///     TcmbEvdsResult advanced_data_result = 
///         tcmb_evds_c_get_advanced_data(
///             data_series, 
///             date, 
///             aggregation_type, 
///             formula, 
///             data_frequency, 
///             api_key, 
///             return_format,
///             ascii_mode,
///         );
///
///
///     // error handling part.
///     if (advanced_data_result.error_type == ParameterError) { /* A Process */ };
///
///     if (advanced_data_result.error_type == InvalidApiKeyOrBadInternetConnection) { /* A Process */ };
/// ```
pub mod error_handling;
mod date_entities;
pub(crate) mod data_series;

use self::error_handling::{ReturnErrorC, handle_return_error};
use self::common_entities::*;
use self::date_entities::*;

use crate::common;
use crate::date::{self, DatePreference};
use crate::error::ReturnError;
use crate::traits::ConvertingToRustEnum;


pub(crate) fn convert_to_ascii(text: &mut String) {

    let english_characters = 
        [('Ç','C'), 
        ('ç', 'c'), 
        ('Ğ', 'G'), 
        ('ğ', 'g'), 
        ('İ', 'I'), 
        ('ı', 'i'), 
        ('Ö', 'O'), 
        ('ö', 'o'), 
        ('Ş', 'S'), 
        ('ş', 's'), 
        ('Ü', 'U'), 
        ('ü', 'u')];


    let a = text.chars()
        .map(|character| { 
            let result = english_characters.iter().find(|chars| chars.0 == character);
        
            let result_char = match result {
                Some(chars) => chars.1,
                None => character,
            };

            result_char
        })
        .map(|character|  {
            if !character.is_ascii() { return '*'; }
        
            character
        });


    *text = a.collect::<String>();
}

pub(crate) fn generate_date_preference(date_data: &str) -> Result<DatePreference, TcmbEvdsResult> {

    let date_preference;
    
    let date_format_type = check_date_format(&date_data)?;

    match date_format_type {
        DateFormatType::Single => {

            let converted_date = date::Date::from(&date_data);
            if let Err(return_error) = converted_date { return Err(handle_return_error(return_error)); }

            date_preference = date::DatePreference::Single(converted_date.unwrap());
        },
        DateFormatType::Multiple => {

            let (rust_start_date, rust_end_date) = parse_dates(&date_data);

            let converted_dates = date::DateRange::from(&rust_start_date, &rust_end_date);
            if let Err(return_error) = converted_dates { return Err(handle_return_error(return_error)); }

            date_preference = date::DatePreference::Multiple(converted_dates.unwrap());
        }
    }

    Ok(date_preference)
}

pub(crate) fn generate_evds(api_key: TcmbEvdsInput, return_format: TcmbEvdsReturnFormat) -> Result<common::Evds, TcmbEvdsResult> {

    let (rust_api_key, api_key_error_state) = api_key.get_input("api_key");
    let rust_return_format = return_format.convert();

    if api_key_error_state {
        return Err(
            TcmbEvdsResult::generate_result(
                rust_api_key,
                ReturnErrorC::ParameterError,
            )
        );
    }

    let handled_api_key = common::ApiKey::from(rust_api_key);
    if let Err(return_error) = handled_api_key { return Err(handle_return_error(return_error)); }
    let valid_api_key = handled_api_key.unwrap();

    Ok(common::Evds::from(valid_api_key, rust_return_format))
}

pub(crate) fn handle_request(request_response: Result<String, ReturnError>) -> TcmbEvdsResult {

    if let Err(return_error) = request_response { return handle_return_error(return_error); } 

    let request_result = request_response.unwrap();
    let error_type = ReturnErrorC::NoError;


    TcmbEvdsResult::generate_result(request_result, error_type)
}

pub(crate) fn return_response(mut response: Result<String, ReturnError>, ascii_mode: bool) -> TcmbEvdsResult {

    if !ascii_mode || response.is_err() { return handle_request(response); } 

    if let Ok(response) = &mut response { convert_to_ascii(response); }

    handle_request(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert() {
        let mut string = String::from("İöüĞÖÜ ©this µthis and 😍this");

        convert_to_ascii(&mut string);

        println!("{}", string);
    }
}
