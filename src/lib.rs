//! Easy and ready access to the database of The Central Bank of The Republic of Turkey (CBRT).
//!
//! This crate provides two main mechanisms for acquiring data from the database:
//!
//! - Operational FFI functions.
//!     +  [`tcmb_evds_c_get_data`](crate::tcmb_evds_c_get_data)
//!     +  [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data)
//!     +  [`tcmb_evds_c_get_data_group`](crate::tcmb_evds_c_get_data_group)
//!     +  [`tcmb_evds_c_get_categories`](crate::tcmb_evds_c_get_categories)
//!     +  [`tcmb_evds_c_get_advanced_data_group`](crate::tcmb_evds_c_get_advanced_data_group)
//!     +  [`tcmb_evds_c_get_series_list`](crate::tcmb_evds_c_get_series_list)
//! - [`evds_c`](crate::evds_c) includes auxiliary enums and structures for the functions to make all of the web service 
//! operations to make users able to utilize these functions in **C language**. 
//!
//! Required auxiliary enums and structures of FFI functions are given in the [`evds_c`](crate::evds_c) module. These 
//! entities make users able to use FFI functions that supply all of the EVDS operations. In addition, most parameters 
//! of the functions are checked to make valid requests. As a result of the 
//! checking, detailed and specified error types are returned to easily handle and fix the errors. Additionally, the 
//! operational functions requires **ascii_mode** to convert a format having non-English characters of the response 
//! text to another format including only English characters. Moreover, the mode as an argument converts non-utf8 
//! characters to asterisk (*) **if there is**. Therefore, the response text becomes safe against non-ascii characters. 
//! To use [`evds_c`](crate::evds_c) in C language, users should build the crate and use both built 
//! *libtcmb_evds_c.so*, *libtcmb_evds_c.dylib* or *libtcmb_evds_c.dll* for multiple platforms and *tcmb_evds_c.h* file 
//! in the target folder or should download one of the pre-built libraries from
//! [github](https://github.com/asari555/tcmb_evds_c).
//!
//! # Usage
//! 
//! For implementations of other functions, please move on their stages.
//! 
//! [`tcmb_evds_c_get_data`](crate::tcmb_evds_c_get_data) usage example.
//! 
//! ```
//! #include "tcmb_evds_c.h"
//! 
//! int main() {
//!     // declaration of required arguments for get data operation.
//!     TcmbEvdsInput data_series;
//!     TcmbEvdsInput date;
//! 
//!     // declaring common variables for creating Evds common struct for each FFI function.
//!     TcmbEvdsInput api_key;
//!     TcmbEvdsReturnFormat return_format;
//! 
//!     bool ascii_mode;
//! 
//! 
//!     data_series.input_ptr = "TP.DK.USD.S";
//!     data_series.string_capacity = strlen(data_series.input_ptr);
//!
//!     date.input_ptr = "13-12-2011, 12-12-2012";
//!     date.string_capacity = strlen(date.input_ptr);
//!
//!     api_key.input_ptr = "VALID_API_KEY";
//!     api_key.string_capacity = strlen(api_key.input_ptr);
//! 
//!     return_format = Csv
//!
//!     ascii_mode = false;
//!
//! 
//!     // due to the fact that data_result may include error, please check it first.
//!     TcmbEvdsResult data_result = tcmb_evds_c_get_data(data_series, date, api_key, return_format, ascii_mode);
//! 
//!     return 0;
//! }
//! ```
//! 
//! 
//! [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data) usage example.
//! 
//! ```
//! #include "tcmb_evds_c.h"
//! 
//! int main() {
//!     // declaration of required arguments for get data operation.
//!     TcmbEvdsInput data_series;
//!     TcmbEvdsInput date;
//!    
//!     // declaration of frequency formulas entities. 
//!     TcmbEvdsAggregationType aggregation_type;
//!     TcmbEvdsFormula formula;
//!     TcmbEvdsDataFrequency data_frequency;
//!    
//!     // declaring common variables for creating Evds common struct for each FFI function.
//!     TcmbEvdsInput api_key;
//!     TcmbEvdsReturnFormat return_format;
//!    
//!     bool ascii_mode;
//!    
//!     
//!     data_series.input_ptr = "TP.DK.USD.A";
//!     data_series.string_capacity = strlen(data_series.input_ptr);
//!    
//!     date.input_ptr = "13-12-2011";
//!     date.string_capacity = strlen(date.input_ptr);
//!      
//!     aggregation_type = End;
//!     formula = Level;
//!     data_frequency = Monthly;
//!    
//!     api_key.input_ptr = "VALID_API_KEY";
//!     api_key.string_capacity = strlen(api_key.input_ptr);
//!    
//!     return_format = Json;
//! 
//!     ascii_mode = false;
//! 
//!    
//!     // due to the fact that data_result may include error, please check it first.
//!     TcmbEvdsResult advanced_data_result = 
//!         tcmb_evds_c_get_advanced_data(
//!             data_series, 
//!             date, 
//!             aggregation_type, 
//!             formula, 
//!             data_frequency, 
//!             api_key, 
//!             return_format,
//!             ascii_mode
//!             );
//!
//!     return 0;
//! }
//! ``` 
//!
//! # Features
//!
//! - Provides all of the EVDS web service operations.
//! - Most of the function parameters in the module are automatically checked inside the functions.
//! - Users are responsible to give valid and correct arguments to related parameters not supporting auto control.
//! - C specific.
//!
//! # User Guide
//!
//! First of all, please take a look at *benioku.md* for Turkish or *readme.md* files before using the crate.
//!
//! Additional resource for understanding some parameters, learning various data series and details of web services are 
//! mentioned in [`kullanim kilavuzu`] for Turkish and [`user guide`].
//!
//! 
//! [`user guide`]: <https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf>
//! [`kullanim kilavuzu`]: <https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Servis_Kullanim_Kilavuzu.pdf>
//!
//! # Appendix
//!
//! To understand *ytl_mode* required in some functions and what is "YTL", please follow [`what is YTL?`].
//!
//! [`what is YTL?`]: <https://en.wikipedia.org/wiki/Revaluation_of_the_Turkish_lira>


// #[deny(missing_docs)]


/// contains two main elements that are used in operations of
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency).
/// 
/// [`ReturnFormat`](crate::common::ReturnFormat) specifies format of the database response and 
/// [`ApiKey`](crate::common::ApiKey) is created to supply valid key to the functions making request operations. 
/// [`Evds`](crate::common::Evds) becomes connection between the functions and both 
/// [`ReturnFormat`](crate::common::ReturnFormat) and [`ApiKey`](crate::common::ApiKey) variables.
///
/// # Usage
///
/// The each element is explained and exampled in their definition parts. This usage illustrates 
/// combined version of common elements on Evds structure to be used in related functions.
///
/// ```
/// # use std::error::Error;
/// # use tcmb_evds_c::common::{ApiKey, ReturnFormat, Evds};
/// # use tcmb_evds_c::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #
///     // Please use a valid key here.
///     let api_key = ApiKey::from("users_valid_key")?;
///
///     let return_format = ReturnFormat::Json;
///
///     let evds = Evds::from(api_key, return_format);
///
/// #   Ok(())
/// # }
/// ```
mod common;
/// contains date elements that are used in some functions of [`evds_basic`](crate::evds_basic) and 
/// [`evds_currency`](crate::evds_currency).
///
/// [`Date`](crate::date::Date) and [`DateRange`](crate::date::DateRange) are created to supply single date and multiple 
/// dates called date range. [`DatePreference`](crate::date::DatePreference) includes [`Date`](crate::date::Date) or 
/// [`DateRange`](crate::date::DateRange) and supplies date options to related functions of 
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency).
///
/// # Usage
///
/// The each element is explained and exampled in their definitions parts.
///
/// ```
/// # use std::error::Error;
/// # use tcmb_evds_c::date::{Date, DateRange, DatePreference};
/// # use tcmb_evds_c::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #
///     // Single date example.
///     let date = Date::from("13-12-2011")?;
///
///     let date_preference = DatePreference::Single(date);
///     
///     
///     // Multiple dates example.
///     let date_range = DateRange::from("13-12-2011", "13-12-2020")?;
///
///     let date_preference = DatePreference::Multiple(date_range);
///
/// #   Ok(())
/// # }
/// ```
mod date;
/// contains specified error options that are returned from the functions of 
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency) to illustrate why the error occurs.
///
/// One of the [`ReturnError`](crate::error::ReturnError) options is returned when something goes wrong with requesting 
/// data or giving parameter to the functions. Therefore, users are able to handle specified error types and to 
/// stringify them in a standard format.
mod error;
/// provides most of the EVDS web services except requesting advanced currency data that means currency data with 
/// frequency formulas.
mod evds_basic;
/// provides only currency operations with methods of [`CurrencySeries`] and [`MultipleCurrencySeries`].
///
/// This module built on two main structures and their methods operating currency services. The basic difference between
/// these structures is number of currency types. [`CurrencySeries`](crate::evds_currency::CurrencySeries) serves
/// methods using a currency type. In contrast, [`MultipleCurrencySeries`](crate::evds_currency::MultipleCurrencySeries)
/// serves a method using more than a currency type to get currencies data.   
///
/// `CurrencySeries` is composed of [`CurrencyCode`], `ExchangeType`, `DatePreference` and `ytl_mode` to supply 
/// adequate information to make requesting data about **a currency** via related **operational methods**. 
///
/// `MultipleCurrencySeries` is composed of [`CurrencyCodes`], `ExchangeType`, `DatePreference` and `ytl_mode` to 
/// supply adequate information to make requesting data about **given currencies** via **an operational method**. 
///
/// Usage schematic and hierarchy of this module:
/// 
/// - [`CurrencySeries`] -> [`get_data`] ( [`Evds`] ) or [`get_advanced_data`] ( [`Evds`], [`AdvancedProcesses`] )
///
///     > `CurrencySeries` requires below struct as a difference:
///     > + [`CurrencyCode`]
///
/// - [`MultipleCurrencySeries`] -> [`get_multiple_data`] ( [`Evds`])
///
///     > `MultipleCurrencySeries` requires below struct as a difference:
///     > + [`CurrencyCodes`]
///
/// Both `CurrencySeries` and `MultipleCurrencySeries` requires common structures and a variable: 
///
/// - [`ExchangeType`]
/// - [`DatePreference`]
/// - `ytl_mode: bool`
///
/// Generic view of operational methods:
///
/// - [`get_data`]
///
/// - [`get_advanced_data`]
/// 
/// - [`get_multiple_data`]
///
/// To use the operational methods, the related structures for the required operation should be built firstly. Then,
/// the methods can be used with the common structure that is [`Evds`].
///
/// *More details of operational functions and their usage are included in their stages.*
///
/// [`CurrencySeries`]: crate::evds_currency::CurrencySeries
/// [`MultipleCurrencySeries`]: crate::evds_currency::MultipleCurrencySeries
///
/// [`Evds`]: crate::common::Evds
/// [`DatePreference`]: crate::date::DatePreference
/// [`ExchangeType`]: crate::evds_currency::ExchangeType
///
/// [`CurrencySeries`]: crate::evds_currency::CurrencySeries
/// [`CurrencyCode`]: crate::evds_currency::CurrencyCode
/// [`AdvancedProcesses`]: crate::evds_currency::frequency_formulas::AdvancedProcesses
///
/// [`MultipleCurrencySeries`]: crate::evds_currency::MultipleCurrencySeries
/// [`CurrencyCodes`]: crate::evds_currency::CurrencyCodes
///
/// [`get_data`]: crate::evds_currency::CurrencySeries::get_data
/// [`get_advanced_data`]: crate::evds_currency::CurrencySeries::get_advanced_data
/// [`get_multiple_data`]: crate::evds_currency::MultipleCurrencySeries::get_multiple_data
mod evds_currency;
mod traits;
/// provides auxiliary enums and structures to FFI to use abilities of the EVDS web services in C language.
///
/// This module has almost the same structural concept with the [`tcmb_evds_c`] crate. [`advanced_entities`], 
/// [`common_entities`] and [`error_handling`]. These modules are responsible to supply required arguments for 
/// related parameters declared with various operational functions.
///
/// Enum and struct of this module includes lucid explanation and its detailed usage example in its 
/// section.
///
/// The related output library of the crate is built in the folder where the `Cargo build --release` command builds. To 
/// see the corresponding function names, enum and struct types in terms of **C**, please look at `target/tcmb_evds_c.h` 
/// automatically built C header file. In addition, to utilize the crate in C language, please first build the crate and 
/// then link one of related **libtcmb_evds_c** libraries and include **tcmb_evd1s_c.h** files in target folder.
///
/// [`tcmb_evds_c`]: crate
/// [`advanced_entities`]: crate::evds_c::advanced_entities
/// [`common_entities`]: crate::evds_c::common_entities
/// [`error_handling`]: crate::evds_c::error_handling
pub mod evds_c;
#[cfg(feature = "async_mode")]
mod request_async;
#[cfg(feature = "sync_mode")]
mod request_sync;


extern crate libc;


use crate::evds_currency::{CurrencySeries, frequency_formulas};
use crate::evds_c::{common_entities::*, error_handling::*};
use crate::evds_c::advanced_entities::{TcmbEvdsAggregationType, TcmbEvdsDataFrequency, TcmbEvdsFormula};
use crate::evds_c::{generate_date_preference, generate_evds, return_response};
use crate::evds_c::data_series::parse_series;
use crate::traits::converting_to_rust_enum::ConvertingToRustEnum;
use libc::c_uint;


/// gets data requested via any valid data series from EVDS.
///
/// # Error
///
/// This function returns error when invalid data series, date, or api key is supplied or there is a bad internet 
/// connection. 
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     TcmbEvdsInput data_series;
///     TcmbEvdsInput date;
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
///     bool ascii_mode;
///     
///     
///     // value assignments.
///     data_series.input_ptr = "TP.DK.USD.S";
///     data_series.string_capacity = strlen(data_series.input_ptr);
///     
///     date.input_ptr = "13-12-2011";
///     date.string_capacity = strlen(date.input_ptr);
///     
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
///     return_format = Csv;
///     
///     ascii_mode = false;
///
///     
///     // requesting data.
///     TcmbEvdsResult data_result = tcmb_evds_c_get_data(data_series, date, api_key, return_format, ascii_mode);
///     
///     
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(data_result)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(data_result) ? "true" : "false");
///   
///     char* data_result_message = calloc(data_result.string_capacity, sizeof(char));
///     memmove(data_result_message, data_result.output_ptr, data_result.string_capacity * sizeof(char));
///    
///     printf("%s", data_result_message);
///
///     free(data_result_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_data(
    data_series: TcmbEvdsInput,
    date: TcmbEvdsInput,
    api_key: TcmbEvdsInput,
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {

    let (rust_data_series, data_series_error_state) = data_series.get_input("data_series");
    let (rust_date, date_error_state) = date.get_input("date");


    let parameter_error = ReturnErrorC::ParameterError;

    if data_series_error_state {
        return TcmbEvdsResult::generate_result(rust_data_series, parameter_error);
    }
    if date_error_state {
        return TcmbEvdsResult::generate_result(rust_date, parameter_error);
    }


    let date_preference_result = generate_date_preference(&rust_date);

    let date_preference = match date_preference_result {
        Ok(preference) => preference,
        Err(error_result) => return error_result,
    };


    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting data from the Tcmb Evds.
    let requested_response = 
    evds_basic::get_data(
        &rust_data_series, 
        &date_preference, 
        &evds
    );


    return_response(requested_response, ascii_mode)
}

/// gets currency data with frequency formulas from EVDS.
///
/// # Error
///
/// This function returns error when invalid currency series, date, aggregation type, formula, data frequency, or api 
/// key is supplied or there is a bad internet connection. 
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     TcmbEvdsInput data_series;
///     TcmbEvdsInput date;
///    
///     TcmbEvdsAggregationType aggregation_type;
///     TcmbEvdsFormula formula;
///     TcmbEvdsDataFrequency data_frequency;
///    
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
///    
///     bool ascii_mode;
///    
///    
///     // value assignments.
///     data_series.input_ptr = "TP.DK.USD.A";
///     data_series.string_capacity = strlen(data_series.input_ptr);
///    
///     date.input_ptr = "13-12-2011";
///     date.string_capacity = strlen(date.input_ptr);
///    
///     aggregation_type = End;
///     formula = Level;
///     data_frequency = Monthly;
///    
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
///    
///     return_format = Json;
///    
///     ascii_mode = false;
///    
///     
///     // requesting data.
///     TcmbEvdsResult advanced_data_result = 
///         tcmb_evds_c_get_advanced_data(
///             data_series, 
///             date, 
///             aggregation_type, 
///             formula, 
///             data_frequency, 
///             api_key, 
///             return_format,
///             ascii_mode
///             );
///    
///    
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(advanced_data_result)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(advanced_data_result) ? "true" : "false");
///   
///     char* advanced_data_result_message = calloc(advanced_data_result.string_capacity, sizeof(char));
///     memmove(
///         advanced_data_result_message, 
///         advanced_data_result.output_ptr, 
///         advanced_data_result.string_capacity * sizeof(char)
///         );
///    
///     printf("%s", advanced_data_result_message);
///
///     free(advanced_data_result_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_advanced_data(
    currency_series: TcmbEvdsInput, 
    date: TcmbEvdsInput,
    aggregation_type: TcmbEvdsAggregationType, 
    formula: TcmbEvdsFormula,
    data_frequency: TcmbEvdsDataFrequency,
    api_key: TcmbEvdsInput,
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {
    
    let (rust_data_series, data_series_error_state) = 
        currency_series.get_input(
            "currency_series"
        );
    let (rust_date, date_error_state) = date.get_input("date");
    let rust_aggregation_type = aggregation_type.convert();
    let rust_formula = formula.convert();
    let rust_data_frequency = data_frequency.convert();

    let parameter_error = ReturnErrorC::ParameterError;

    if data_series_error_state {
        return TcmbEvdsResult::generate_result(rust_data_series, parameter_error);
    }
    if date_error_state {
        return TcmbEvdsResult::generate_result(rust_date, parameter_error);
    }


    let advanced_processes = 
        frequency_formulas::AdvancedProcesses::from(
            rust_aggregation_type, 
            rust_formula, 
            rust_data_frequency
        );

    
    let data_series_parts = parse_series(&rust_data_series);

    if let Err(return_error) = data_series_parts {  return handle_return_error(return_error); };
    let data_series_parts  = data_series_parts.unwrap();


    let date_preference_result = generate_date_preference(&rust_date);

    let date_preference = match date_preference_result {
        Ok(preference) => preference,
        Err(error_result) => return error_result,
    };


    let currency_series = CurrencySeries {
        ytl_mode: data_series_parts.ytl_mode,
        exchange_type: data_series_parts.exchange_type,
        currency_code: data_series_parts.currency_code,
        date_preference
    };


    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting advanced currency data from the Tcmb Evds.
    let requested_response =
        currency_series.get_advanced_data(
            &evds, 
            &advanced_processes
        );
    

    return_response(requested_response, ascii_mode)
}

/// gets all series data related given data group from EVDS.
///
/// # Error
///
/// This function returns error when invalid data_group, date, or api key is supplied or there is a bad internet 
/// connection.
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     TcmbEvdsInput data_group;
///     TcmbEvdsInput date;
///     
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
/// 
///     bool ascii_mode;
/// 
/// 
///     // value assignments.
///     data_group.input_ptr = "bie_yssk";
///     data_group.string_capacity = strlen(data_group.input_ptr);
/// 
///     date.input_ptr = "01-06-2017,07-09-2017";
///     date.string_capacity = strlen(date.input_ptr);
/// 
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
/// 
///     return_format = Json;
/// 
///     ascii_mode = false;
/// 
/// 
///     // requesting data.
///     TcmbEvdsResult data_group = 
///         tcmb_evds_c_get_data_group(
///             data_group, 
///             date, 
///             api_key, 
///             return_format,
///             ascii_mode
///             );
/// 
///
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(data_group)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(data_group) ? "true" : "false");
///   
///     char* data_group_message = calloc(data_group.string_capacity, sizeof(char));
///     memmove(data_group_message, data_group.output_ptr, data_group.string_capacity * sizeof(char));
///    
///     printf("%s", data_group_message);
///
///     free(data_group_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_data_group(
    data_group: TcmbEvdsInput, 
    date: TcmbEvdsInput, 
    api_key: TcmbEvdsInput, 
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {

    let (rust_data_group, data_group_error_state) = data_group.get_input("data_group");
    let (rust_date, date_error_state) = date.get_input("date");

    let parameter_error = ReturnErrorC::ParameterError;

    if data_group_error_state {
        return TcmbEvdsResult::generate_result(rust_data_group, parameter_error);
    }
    if date_error_state {
        return TcmbEvdsResult::generate_result(rust_date, parameter_error);
    }


    let date_preference_result = generate_date_preference(&rust_date);

    let date_preference = match date_preference_result {
        Ok(preference) => preference,
        Err(error_result) => return error_result,
    };


    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting data group from the Tcmb Evds.
    let requested_response = 
    evds_basic::get_data_group(
        &rust_data_group, 
        &date_preference, 
        &evds
    );


    return_response(requested_response, ascii_mode)
}

/// gets categories list from EVDS.
///
/// # Error
///
/// This function returns error when invalid api key is supplied or there is a bad internet connection.
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
/// 
///     bool ascii_mode;
/// 
///
///     // value assignments.
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
/// 
///     return_format = Json;
/// 
///     ascii_mode = false;
/// 
/// 
///     // requesting data.
///     TcmbEvdsResult categories = tcmb_evds_c_get_categories(api_key, return_format, ascii_mode);
/// 
/// 
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(categories)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(categories) ? "true" : "false");
///   
///     char* categories_message = calloc(categories.string_capacity, sizeof(char));
///     memmove(categories_message, categories.output_ptr, categories.string_capacity * sizeof(char));
///    
///     printf("%s", categories_message);
///
///     free(categories_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_categories(
    api_key: TcmbEvdsInput, 
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {

    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting categories data from the Tcmb Evds.
    let requested_response = evds_basic::get_categories(&evds);


    return_response(requested_response, ascii_mode)
}

/// gets data groups from EVDS.
///
/// # Error
///
/// This function returns error when invalid mode, code, or api key is supplied or there is a bad internet connection. 
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     unsigned int mode;
///     TcmbEvdsInput code;   
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
///     bool ascii_mode;
///    
///    
///     // value assignments.
///     mode = 1;
///    
///     code.input_ptr = "bie_yssk";
///     code.string_capacity = strlen(code.input_ptr);
///     
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = strlen(api_key.input_ptr);
///    
///     return_format = Json;
///     
///     ascii_mode = false;
///    
///    
///     // requesting data.
///     TcmbEvdsResult advanced_data_group = 
///         tcmb_evds_c_get_advanced_data_group(
///             mode, 
///             code, 
///             api_key, 
///             return_format, 
///             ascii_mode
///             );
///     
///    
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(advanced_data_group)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(advanced_data_group) ? "true" : "false");
///   
///     char* advanced_data_group_message = calloc(advanced_data_group.string_capacity, sizeof(char));
///     memmove(
///         advanced_data_group_message, 
///         advanced_data_group.output_ptr, 
///         advanced_data_group.string_capacity * sizeof(char)
///         );
///    
///     printf("%s", advanced_data_group_message);
///
///     free(advanced_data_group_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_advanced_data_group(
    mode: c_uint,
    code: TcmbEvdsInput,
    api_key: TcmbEvdsInput,
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {

    let (rust_code, code_error_state) = code.get_input("code");

    if code_error_state {
        return TcmbEvdsResult::generate_result(rust_code, ReturnErrorC::ParameterError);
    }
    

    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting advanced data group from the Tcmb Evds.
    let requested_response = evds_basic::get_advanced_data_group(mode, &rust_code, &evds);


    return_response(requested_response, ascii_mode)
}

/// gets series list from EVDS.
///
/// # Error
///
/// This function returns error when invalid code, or api_key is given or there is a bad internet connection.
///
/// # Example
///
/// ```C
///
/// #include "tcmb_evds_c.h"
///
///
/// int main() {
///
///     // declaration of required arguments.
///     TcmbEvdsInput code;   
///     TcmbEvdsInput api_key;
///     TcmbEvdsReturnFormat return_format;
///     bool ascii_mode;
///    
///    
///     // value assignments.
///     code.input_ptr = "bie_yssk";
///     code.string_capacity = 8;
///    
///     api_key.input_ptr = "VALID_API_KEY";
///     api_key.string_capacity = 10;
///    
///     return_format = Csv;
///    
///     ascii_mode = false;
///    
///    
///     // requesting data.
///     TcmbEvdsResult series_list = tcmb_evds_c_get_series_list(code, api_key, return_format, ascii_mode);
///    
///
///     // handling error and printing the result.
///     if (!tcmb_evds_c_is_error(series_list)) { printf("\nNO ERROR!\n"); };
///     printf("\nError: %s", tcmb_evds_c_is_error(series_list) ? "true" : "false");
///   
///     char* series_list_message = calloc(series_list.string_capacity, sizeof(char));
///     memmove(series_list_message, series_list.output_ptr, series_list.string_capacity * sizeof(char));
///    
///     printf("%s", series_list_message);
///
///     free(series_list_message);
///
///     return 0;
/// }
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_get_series_list(
    code: evds_c::common_entities::TcmbEvdsInput, 
    api_key: TcmbEvdsInput, 
    return_format: TcmbEvdsReturnFormat,
    ascii_mode: bool
) -> TcmbEvdsResult {

    let (rust_code, code_error_state) = code.get_input("code");

    if code_error_state {
        return TcmbEvdsResult::generate_result(rust_code,  ReturnErrorC::ParameterError);
    }
   

    let evds_result = generate_evds(api_key, return_format);

    let evds = match evds_result {
        Ok(evds) => evds,
        Err(error_result) => return error_result,
    };


    // Requesting series list from the Tcmb Evds.
    let requested_response = evds_basic::get_series_list(&rust_code, &evds);

    
    return_response(requested_response, ascii_mode)
}

/// provides users an ability to check whether the result includes error or not. 
///
/// # Example
/// 
/// ```C
///     // requesting data.
///     TcmbEvdsResult series_list = tcmb_evds_c_get_series_list(code, api_key, return_format, ascii_mode);
///
///
///     // handling error.
///     printf("\nError: %s", tcmb_evds_c_is_error(series_list) ? "true" : "false");
/// ```
#[no_mangle]
pub extern "C" fn tcmb_evds_c_is_error(result: TcmbEvdsResult) -> bool {

    if let ReturnErrorC::NoError = result.error_type { return false; }
    
    true
}