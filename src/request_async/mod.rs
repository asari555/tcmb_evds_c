#[cfg(feature = "async_mode")]
use curl::easy::{Easy2, Handler, WriteError};

#[cfg(feature = "async_mode")]
use crate::error::ReturnError;


// TESTED
#[cfg(feature = "async_mode")]
struct Collector(Vec<u8>);

#[cfg(feature = "async_mode")]
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}


/// requests required data from server via given url in async mode.
///
/// This function is fundamental and at the bottom level of the requesting hierarchy.  
#[cfg(feature = "async_mode")]
pub(crate) fn do_request(url_format: &str) -> Result<String, ReturnError> {
    let mut handle = Easy2::new(Collector(Vec::new()));
    
    if let Err(_) = handle.get(true) {
        return Err(ReturnError::UnableToRequest)
    }
    if let Err(_) = handle.url(url_format) {
        return Err(ReturnError::UnableToSetUrl);
    }

    
    // Applying request is repeated 3 times if the operation does not work properly. In the last turn if the perform()
    // function ends up with an error, an error is returned from the loop. Otherwise, successful operation breaks the 
    // loop.
    let mut perform_result;

    for element in 0..3 {
        perform_result = handle.perform();

        if perform_result.is_ok() { break; }

        if element != 2 { continue; }

        return Err(ReturnError::FailedToApplyRequest);
    }


    match handle.response_code() {
        Ok(number) => {
            if number != 200 {
                return Err(ReturnError::RequestDenied)
            }
        },
        Err(_) => return Err(ReturnError::NotFound),
    }

    let contents = handle.get_ref();
    let response = String::from_utf8_lossy(&contents.0);
    
    Ok(response.to_string())
}
