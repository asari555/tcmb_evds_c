use crate::error::ReturnError;
use super::common_entities::TcmbEvdsResult;

/// There is a **'C'** letter at the end of the enum name. This comes from C language. The name means that 
/// `ReturnError` for C.
#[derive(Debug)]
#[repr(C)]
pub enum ReturnErrorC {
    NoError,
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
    ResponseError,
    EmptyResponse,
    ForbiddenRequest,
    MissingNumberInDateData,
    MissingDashInDateData,
    MissingCommaInDateData,
    DateDataExceedingLengthLimit,
    UndefinedDateDataFormat,
    ParameterError,
}

/// converts `error::ReturnError` into `error_handling::ReturnErrorC` with error message.
fn convert_return_error(return_error: ReturnError) -> (ReturnErrorC, String) {

    let error;
    let error_message;

    match return_error {
        ReturnError::InvalidApiKeyOrBadInternetConnection => {

            error = ReturnErrorC::InvalidApiKeyOrBadInternetConnection;

            error_message = ReturnError::InvalidApiKeyOrBadInternetConnection.to_string();
        },
        ReturnError::BadInternetConnection => {

            error = ReturnErrorC::BadInternetConnection;

            error_message = ReturnError::BadInternetConnection.to_string();
        },
        ReturnError::BadInternetConnectionOrInvalidUrl => {

            error = ReturnErrorC::BadInternetConnectionOrInvalidUrl;

            error_message = ReturnError::BadInternetConnectionOrInvalidUrl.to_string();
        },
        ReturnError::InvalidUrl => {

            error = ReturnErrorC::InvalidUrl;

            error_message = ReturnError::InvalidUrl.to_string();
        },
        ReturnError::InvalidSeries => {

            error = ReturnErrorC::InvalidSeries;

            error_message = ReturnError::InvalidSeries.to_string();
        },
        ReturnError::EmptyParameter => {

            error = ReturnErrorC::EmptyParameter;

            error_message = ReturnError::EmptyParameter.to_string();
        },
        ReturnError::InvalidDate => {

            error = ReturnErrorC::InvalidDate;

            error_message = ReturnError::InvalidDate.to_string();
        },
        ReturnError::EmptyExchangeType => {

            error = ReturnErrorC::EmptyExchangeType;

            error_message = ReturnError::EmptyExchangeType.to_string();
        },
        ReturnError::EmptyCurrencyCodes => {

            error = ReturnErrorC::EmptyCurrencyCodes;

            error_message = ReturnError::EmptyCurrencyCodes.to_string();
        },
        ReturnError::SingleExchangeTypeExpected => {

            error = ReturnErrorC::SingleExchangeTypeExpected;

            error_message = ReturnError::SingleExchangeTypeExpected.to_string();
        },
        ReturnError::SingleDateExpected => {

            error = ReturnErrorC::SingleDateExpected;

            error_message = ReturnError::SingleDateExpected.to_string();
        },
        ReturnError::MultipleDateExpected => {

            error = ReturnErrorC::MultipleDateExpected;

            error_message = ReturnError::MultipleDateExpected.to_string();
        },
        ReturnError::RequestDenied => {

            error = ReturnErrorC::RequestDenied;

            error_message = ReturnError::RequestDenied.to_string();
        },
        ReturnError::NotFound => {

            error = ReturnErrorC::NotFound;

            error_message = ReturnError::NotFound.to_string();
        },
        ReturnError::UnableToRequest => {

            error = ReturnErrorC::UnableToRequest;

            error_message = ReturnError::UnableToRequest.to_string();
        },
        ReturnError::UnableToSetUrl => {

            error = ReturnErrorC::UnableToSetUrl;

            error_message = ReturnError::UnableToSetUrl.to_string();
        },
        ReturnError::FailedToApplyRequest => {

            error = ReturnErrorC::FailedToApplyRequest;

            error_message = ReturnError::FailedToApplyRequest.to_string();
        },
        ReturnError::FailedToSaveReceivedData => {

            error = ReturnErrorC::FailedToSaveReceivedData;

            error_message = ReturnError::FailedToSaveReceivedData.to_string();
        },
        ReturnError::ResponseError(message) => {

            error = ReturnErrorC::ResponseError;

            error_message = message;
        }, 
        ReturnError::EmptyResponse => {

            error = ReturnErrorC::EmptyResponse;

            error_message = ReturnError::EmptyResponse.to_string();
        },
        ReturnError::ForbiddenRequest => {

            error = ReturnErrorC::ForbiddenRequest;

            error_message = ReturnError::ForbiddenRequest.to_string();
        },
    }

    (error, error_message)
}

pub(crate) fn handle_return_error(return_error: ReturnError) -> TcmbEvdsResult {

    let (error_type, error_message) = convert_return_error(return_error);

    TcmbEvdsResult::generate_result(error_message, error_type)
}
