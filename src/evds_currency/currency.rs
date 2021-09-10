use crate::error::ReturnError;
#[cfg(feature = "async_mode")]
use crate::request_async;
#[cfg(feature = "sync_mode")]
use crate::request_sync;

/// checks empty empty response to handle possible error.
///
/// # Error
///
/// This function returns an error if the response against incorrect request is empty.
fn check_empty_response(response: &str) -> Result<(), ReturnError> {
    if response.is_empty() { return Err(ReturnError::EmptyResponse); }
    Ok(())
}

/// makes the required request and is compatible with async programming.
///
/// This function is configured for evds currency operations.
#[cfg(feature = "async_mode")]
fn make_request_async(url: &str) -> Result<String, ReturnError> {
    let response = request_async::do_request(&url)?;
    check_empty_response(&response)?;
    Ok(response)
}

/// makes the required request and is compatible with async programming.
///
/// This function is configured for evds currency operations.
#[cfg(feature = "sync_mode")]
fn make_request_sync(url: &str) -> Result<String, ReturnError> {
    let response = request_sync::do_request(&url)?;
    check_empty_response(&response)?;
    Ok(response)
}

/// Combined version of *make_request_async* and *make_request_sync* functions.
///
/// The most important feature of this function is that the functionality of the function can be changed when 
/// the crate is compiled according to preferred compiling feature, thanks to the conditional compiling of Rust.
///
/// This function is configured for evds currency operations.
pub(crate) fn make_request(url: &str) -> Result<String, ReturnError> {
    #[cfg(feature = "async_mode")]
    return make_request_async(url);
    #[cfg(feature = "sync_mode")]
    return make_request_sync(url);
}
