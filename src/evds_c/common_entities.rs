use std::{os::raw::c_char, ffi::CStr};

use libc::{c_uchar, c_ulong};

use super::error_handling::ReturnErrorC;
use crate::traits::{converting_to_rust_enum::ConvertingToRustEnum, enum_specific::EnumSpecific};
use crate::common::ReturnFormat;

/// contains the text of the response to the submitted request or information about an error that should be easily read 
/// and handled in C language. 
///
/// To read the response text the string capacity should be used.
///
/// The error type becomes `ReturnErrorC::NoError` when there is no error. Otherwise, it returns a related error type 
/// with the given error.
#[repr(C)]
pub struct TcmbEvdsResult {
    pub output_ptr: *mut c_uchar,
    pub string_capacity: c_ulong,
    pub error_type: ReturnErrorC,
}

impl TcmbEvdsResult {
    /// generates tcmb evds result type result.
    ///
    /// # Error
    ///
    /// This function returns error message when error_state becomes true and the given request_result contains error 
    /// message.
    pub(crate) fn generate_result(request_result: String, error_type: ReturnErrorC) -> TcmbEvdsResult {
        
        let error_message_length = request_result.len();
            
        let boxed_error = request_result.into_boxed_str();
        let sendable_error = Box::leak(boxed_error).as_mut_ptr();
            
        let result = TcmbEvdsResult { 
            output_ptr: sendable_error,
            string_capacity: error_message_length as c_ulong,
            error_type,
        };
        
        return result;
    }
}

/// includes an input string pointer and its size to easily read an input string by Rust language.
#[repr(C)]
pub struct TcmbEvdsInput {
    pub input_ptr: *const c_char,
    pub string_capacity: c_ulong,
}

impl TcmbEvdsInput {
    /// generates Rust string with given c_char pointer and its length.
    ///
    /// # Error
    /// 
    /// This function returns an error string and error state true in a tuple structure when the string taken from 
    /// C could not be converted to Rust string slice. 
    ///
    /// Error message contains the `parameter name` as an error indicator.
    pub(crate) fn get_input(&self, parameter_name: &str) -> (String, bool) {
        let c_data_series = unsafe { CStr::from_ptr(self.input_ptr) };

        let result_string;

        let error_state;

        match  c_data_series.to_str() {
            Ok(series) => {
                let rust_string = &series[..self.string_capacity as usize];

                result_string = String::from(rust_string);

                error_state = false;
            }
            _ => {
                result_string = format!("Error: There is a problem with given {} parameter.", parameter_name);

                error_state = true;
            }
        }

        (result_string, error_state)
    }
}


/// is used to specify the return format of the required response.
#[repr(C)]
pub enum TcmbEvdsReturnFormat {
    Csv,
    Json,
    Xml,
}

impl ConvertingToRustEnum<ReturnFormat> for TcmbEvdsReturnFormat {
    /// returns `Json` option by default.
    fn convert(&self) -> ReturnFormat {
        match self {
            TcmbEvdsReturnFormat::Csv => return ReturnFormat::Csv,
            TcmbEvdsReturnFormat::Xml => return ReturnFormat::Xml,
            _ => return ReturnFormat::Json,
        }
    }
}

impl EnumSpecific for ReturnFormat {}
