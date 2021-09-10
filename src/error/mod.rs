use std::{cmp, error, fmt};


/// contains specified error options returned from various [`tcmb_evds_c`](crate) operations.
///
/// This enum is returned with an error option to specify what goes wrong.
///
/// Only **ResponseError** option of this enum contains an error message which is a returned response 
/// against incorrect request.
#[derive(Debug)]
pub(crate) enum ReturnError {
    InvalidApiKeyOrBadInternetConnection,
    BadInternetConnection,
    BadInternetConnectionOrInvalidUrl,
    InvalidUrl,
    InvalidSeries,
    EmptyParameter,
    InvalidDate,
    EmptyExchangeType,
    EmptyCurrencyCodes,
    SingleExchangeTypeExpected,
    SingleDateExpected,
    MultipleDateExpected,
    RequestDenied,
    NotFound,
    UnableToRequest,
    UnableToSetUrl,
    FailedToApplyRequest,
    FailedToSaveReceivedData,
    ResponseError(String),
    EmptyResponse,
    ForbiddenRequest,
}

impl ReturnError {
    /// stringifies returned error in a standard format.
    pub(crate) fn to_string(&self) -> String {
        match self {
            ReturnError::InvalidApiKeyOrBadInternetConnection => return "Error: Invalid api key or bad internet connection.".to_string(),
            ReturnError::BadInternetConnection => return "Error: Bad internet connection.".to_string(),
            ReturnError::BadInternetConnectionOrInvalidUrl => return "Error: Bad internet connection or invalid url.".to_string(),
            ReturnError::InvalidUrl => return "Error: Invalid url.".to_string(),
            ReturnError::InvalidSeries => return "Error: Invalid series.".to_string(),
            ReturnError::EmptyParameter => return "Error: Empty parameter.".to_string(),
            ReturnError::InvalidDate => return "Error: Invalid date.".to_string(),
            ReturnError::EmptyExchangeType => return "Error: Empty exchange type.".to_string(),
            ReturnError::EmptyCurrencyCodes => return "Error: Empty currency codes.".to_string(),
            ReturnError::SingleExchangeTypeExpected => return "Error: Single exchange type expected.".to_string(),
            ReturnError::SingleDateExpected => return "Error: Single date expected.".to_string(),
            ReturnError::MultipleDateExpected => return "Error: Multiple date expected.".to_string(),
            ReturnError::RequestDenied => return "Error: Request denied.".to_string(),
            ReturnError::NotFound => return "Error: 404 not found.".to_string(),
            ReturnError::UnableToRequest => return "Error: Unable to ask for a HTTP GET request.".to_string(),
            ReturnError::UnableToSetUrl => return "Error: Unable to appropriately set url.".to_string(),
            ReturnError::FailedToApplyRequest => return "Error: Failed to apply HTTP request.
            \nHelp: please check the internet connection or the validity of given url.".to_string(),
            ReturnError::FailedToSaveReceivedData => return "Error: Failed to save received data.".to_string(),
            ReturnError::ResponseError(message) => return message.to_owned(),
            ReturnError::EmptyResponse => return "Error: Empty page returned.".to_string(),
            ReturnError::ForbiddenRequest => return "Error: The request is forbidden.
            \nHelp: please check given data series is wether single or not.".to_string(),
        }
    }
}

impl cmp::PartialEq for ReturnError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for ReturnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {         
        write!(f, "({})", self.to_string())
    }
}

impl error::Error for ReturnError {}
