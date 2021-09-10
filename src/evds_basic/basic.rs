use crate::error::ReturnError;
#[cfg(feature = "async_mode")]
use crate::request_async;
#[cfg(feature = "sync_mode")]
use crate::request_sync;


/// provides function options to divert the flow of [`check_response`](fn@check_response) in the specific make 
/// request functions only utilized for ['get_data_group'](fn@get_data_group) and 
/// ['get_series_list'](fn@get_series_list) functions. 
pub(crate) enum Function{
    GetDataGroup,
    GetSeriesList,
    OneOfOtherFunctions,
}


/// checks emptiness of given parameters.
///
/// # Error
///
/// This function returns an error if one of given parameters is empty.
pub(crate) fn check_emptiness(data: &str) -> Result<(), ReturnError> {
    
    if data.is_empty() { return Err(ReturnError::EmptyParameter) };
    
    Ok(())
}

/// When getting data group, system may respond an error message due to a mistake. So, this function
/// handles the response and if an error occurs the function returns response error 
/// containing error message.
/// 
/// # Error
///
/// This function returns an error if the response message contains known error.
fn handle_response_error(response_message: &str) -> Result<(), ReturnError> {
    
    let mut words: Vec<&str> = response_message.split(' ').collect();
    
    if let Some(firs_word) = words.iter().next() {
        let lower_case = firs_word.to_lowercase();
        if lower_case != "no" { return Ok(()); }
    }

    if let Some(last_word) = words.iter().next_back() {
        if *last_word == "!" || *last_word == "'" || *last_word == "," || *last_word == ";" {
            words.pop();
        }
    }

    let mut output = String::from("Error: ");
    for word in words {
        output.push_str(word);
        output.push(' ');
    }

    output.pop();
    output.push_str(".");

    let error = ReturnError::ResponseError(output);

    Err(error)
}

/// checks the response opposed to the request is wether error/empty or not.
///
/// According to given function as a parameter, the error checking flow is changed. 
///
/// # Error
///
/// This function returns an error if the response message contains known and specified error. 
pub(crate) fn check_response(response: &str, function: Function) -> Result<(), ReturnError> {
    match function {
        Function::GetDataGroup => { return handle_response_error(&response); },
        Function::GetSeriesList => { 
            // This part returns error if response contains xml_empty or json_empty given below.
            let xml_empty = "<document></document>";
            let json_empty = "[]";

            if response == xml_empty || response == json_empty {
                return Err(ReturnError::NotFound);
            } 
        },
        Function::OneOfOtherFunctions => {}
    }

    if response.is_empty() {
        return Err(ReturnError::EmptyResponse);
    }

    Ok(())
} 

/// provides special make request functionality to [`get_data_group`](fn@get_data_group) and 
/// [`get_series_list`](fn@get_series_list), and more generally to rest of functions.
///
/// The rest of functions utilize this function to check the response is wether empty or not. Additionally, the given
/// two function also use this function for the same purpose.
///
/// This function is applicable for async operations and configured for evds basic operations.
#[cfg(feature = "async_mode")]
fn make_request_async(url: &str, function: Function) -> Result<String, ReturnError> {
    
    let response = request_async::do_request(&url)?;
    
    check_response(&response, function)?;
    
    Ok(response)
}

/// provides special make_request functionality especially to both [`get_data_group`](fn@get_data_group) and 
/// ['get_series_list'](fn@get_series_list), and more generally to rest of functions.
///
/// The rest of functions utilize this function to check the response is wether empty or not. Additionally, the given
/// two function also use this function for the same purpose.
///
/// This function is applicable for sync operations and configured for evds basic operations.
#[cfg(feature = "sync_mode")]
fn make_request_sync(url: &str, function: Function) -> Result<String, ReturnError> {
    
    let response = request_sync::do_request(&url)?;

    check_response(&response, function)?;

    Ok(response)
}

/// Combined version of *make_request_async* and *make_request_sync* functions.
///
/// The most important feature of this function is that the functionality of the function can be changed when 
/// the crate is compiled according to preferred compiling feature, thanks to the conditional compiling of Rust.
///
/// This function is configured for evds currency operations.
pub(crate) fn make_request(url: &str, function: Function) -> Result<String, ReturnError> {
    #[cfg(feature = "async_mode")]
    return make_request_async(url, function);

    #[cfg(feature = "sync_mode")]
    return make_request_sync(url, function);
}
