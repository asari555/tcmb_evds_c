#[cfg(feature = "sync_mode")]
use curl::easy::Easy;

#[cfg(feature = "sync_mode")]
use crate::error::ReturnError;


/// requests required data from server via given url in sync mode.
///
/// This function is fundamental and at the bottom level of the requesting hierarchy.  
#[cfg(feature = "sync_mode")]
pub(crate) fn do_request(url_format: &str) -> Result<String, ReturnError> {
    let mut buf = Vec::new();
    let mut handle = Easy::new();

    if let Err(_) = handle.url(url_format) {
        return Err(ReturnError::UnableToSetUrl);
    }

    {
        let mut transfer = handle.transfer();
        if let Err(_) = transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }) {
            return Err(ReturnError::FailedToSaveReceivedData);
        }

        
        // Applying request is repeated 3 times if the operation does not work properly. In the last turn if the 
        // perform() function ends up with an error, an error is returned from the loop. Otherwise, successful operation 
        // breaks the loop.
        let mut perform_result;

        for element in 0..3 {
            perform_result = transfer.perform();

            if perform_result.is_ok() { break; }

            if element != 2 { continue; }
            
            return Err(ReturnError::FailedToApplyRequest);
        }
    }

    let response = String::from_utf8_lossy(&buf);
    
    if response.is_empty() {
        return Err(ReturnError::NotFound);
    }

    Ok(response.to_string())
}
