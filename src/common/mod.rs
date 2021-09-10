use std::cmp;

use crate::error::ReturnError;
use crate::traits::{self, MakingUrlFormat};

#[cfg(feature = "async_mode")]
use crate::request_async;

#[cfg(feature = "sync_mode")]
use crate::request_sync;


/// provides users an option menu to choose one of the return format.
///
/// Users are expected to use appropriate format for related request.
pub(crate) enum ReturnFormat {
    /// Comma Separated Values format.
    Csv,
    /// Java Script Object Notation format.
    Json,
    /// Extensible Markup Language format.
    Xml,
} 

impl ToString for ReturnFormat {
    /// returns stringified version of return format option that is appropriate for url usage. 
    fn to_string(&self) -> String {
        match self {
            &Self::Csv => String::from("csv"),
            &Self::Json => String::from("json"),
            &Self::Xml => String::from("xml"),
        }
    }
}

impl traits::MakingUrlFormat for ReturnFormat {
    /// generates required url representation of return format.
    fn generate_url_format(&self) -> String {
        format!("type={}", self.to_string())
    }
}


/// is the container of the api key validated.
///
/// To check validity of the given api key, users need to create an api key variable via 
/// [`ApiKey::from`](fn@ApiKey::from).
#[derive(Debug)]
pub(crate) struct ApiKey(String);

impl<'a> ApiKey {
    fn change(&mut self, new_key: &'a str) -> Result<(), ReturnError> {
        let api_key = ApiKey(new_key.to_string());

        api_key.is_api_key_valid()?;
        
        self.0 = new_key.to_string();

        Ok(())
    }

    #[cfg(feature = "async_mode")]
    fn check_api_key_validity_async(reference_url: String) -> Result<(), ReturnError> {
        match request_async::do_request(&reference_url) {
            Ok(_) => Ok(()),
            Err(_) => Err(ReturnError::InvalidApiKeyOrBadInternetConnection),
        }
    }

    #[cfg(feature = "sync_mode")]
    fn check_api_key_validity_sync(reference_url: String) -> Result<(), ReturnError> {
        match request_sync::do_request(&reference_url) {
            Ok(_) => Ok(()),
            Err(_) => Err(ReturnError::InvalidApiKeyOrBadInternetConnection),
        }
    }

    fn is_api_key_valid(&self) -> Result<(), ReturnError> {
        // The string below is divided into two due to the convention of horizontal width which is 120 characters. 
        let reference_url = 
        format!(
            "https://evds2.tcmb.gov.tr/service/evds/series=TP.DK.USD.S.YTL{}&key={}", 
            "&startDate=13-12-2011&endDate=13-12-2011&type=json",
            self.0,
        );
    
        #[cfg(feature = "async_mode")]
        return ApiKey::check_api_key_validity_async(reference_url);

        #[cfg(feature = "sync_mode")]
        return ApiKey::check_api_key_validity_sync(reference_url);
    }

    fn get(&self) -> &str {
        &self.0
    }

    /// is needed to automatically check validation of api key for new instance.
    /// 
    /// The internet connection is required to achieve the task.
    ///
    /// # Error
    ///
    /// The function will return error if given api key is invalid or there is a bad internet connection.
    ///
    /// # Examples
    ///
    /// ```
    ///     use tcmb_evds_c::common::ApiKey;
    ///     use tcmb_evds_c::error::ReturnError;
    ///
    ///
    ///     // If user key entered is valid, the function creates api_key variable. 
    ///     // Otherwise, returns one of ReturnError options.
    ///     // The function returns an error unless users write their own valid api key.
    ///     let result = ApiKey::from("users_key".to_string());
    ///
    ///
    ///     // Users can handle error in a different way.
    ///     let api_key = match result {
    ///         Err(return_error) => { 
    ///             println!("{}", return_error.to_string()); 
    ///             return;
    ///         },
    ///         Ok(api_key) => api_key,
    ///     };
    /// ```
    pub(crate) fn from(key: String) -> Result<ApiKey, ReturnError> {
        let api_key = ApiKey(key);

        api_key.is_api_key_valid()?;

        Ok(api_key)
    }
}

impl cmp::PartialEq for ApiKey {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl traits::MakingUrlFormat for ApiKey {
    fn generate_url_format(&self) -> String {
        format!("key={}", self.0)
    }
}


/// is composed of created [`ApiKey`](struct@ApiKey) and [`ReturnFormat`](crate::common::ReturnFormat) variables.
///
/// This struct is common for each function that this crate provides.
pub(crate) struct Evds {
    api_key: ApiKey,
    return_format: ReturnFormat,
}

impl<'a> Evds {
    /// creates an Evds object from given [`ApiKey`](struct@ApiKey) and [`ReturnFormat`](enum@ReturnFormat). 
    /// 
    /// # Examples
    ///
    /// ```
    /// # use tcmb_evds_c::error::ReturnError;
    /// # use tcmb_evds_c::common::{ApiKey, ReturnFormat};
    ///     use tcmb_evds_c::common::Evds;
    /// #
    /// # // If user key entered is valid, the function creates api_key variable. 
    /// # // Otherwise, returns one of ReturnError.
    /// # // The function returns an error unless users write their own api key valid.
    /// # let result = ApiKey::from("users_key".to_string());
    /// #
    /// #
    /// # // Users can handle error in a different way.
    /// # let api_key = match result {
    /// #   Err(return_error) => { 
    /// #       println!("{}", return_error.to_string()); 
    /// #       return;
    /// #   },
    /// #   Ok(api_key) => api_key,
    /// # };
    ///
    ///
    ///     // Valid api_key is required.
    ///     let evds = Evds::from(api_key, ReturnFormat::Json);
    /// ```
    pub(crate) fn from(api_key: ApiKey, return_format: ReturnFormat) -> Evds {
        Evds {
            api_key,
            return_format,
        }
    }

    /// changes api key contained in Evds object if and only if the given key is valid.
    ///
    /// The internet connection is required to achieve the task.
    ///
    /// # Error
    /// 
    /// The function will return error if given api key is invalid or there is a bad internet connection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # use tcmb_evds_c::error::ReturnError;
    ///     use tcmb_evds_c::common::{ApiKey, ReturnFormat, Evds};
    /// 
    /// 
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # let api_key = ApiKey::from("users_key")?;
    ///     let mut evds = Evds::from(api_key, ReturnFormat::Json);
    ///          
    ///          
    ///     evds.change_api_key("user's_new_key")?;
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) fn change_api_key(&mut self, api_key: &str) -> Result<(), ReturnError> {

        self.api_key.change(api_key)?;

        Ok(())
    }

    /// changes return format inside of an [`Evds`](struct@Evds) variable.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tcmb_evds_c::error::ReturnError;
    /// # use tcmb_evds_c::common::{ApiKey, ReturnFormat};
    ///     use tcmb_evds_c::common::Evds;
    /// # let result = ApiKey::from("users_key".to_string());
    /// #
    /// # if let Err(_) = result {
    /// #   return;      
    /// # }
    /// #
    /// # let api_key = result.unwrap();
    /// #
    /// #
    /// # let mut evds = Evds::from(api_key, ReturnFormat::Json);
    ///
    ///
    ///     evds.change_return_format(ReturnFormat::Xml);
    /// ```
    pub(crate) fn change_return_format(&mut self, return_format: ReturnFormat) {
        self.return_format = return_format;
    }

    /// generates url format of api key.
    pub(crate) fn get_api_key_as_url(&self) -> String {
        self.api_key.generate_url_format()
    }

    /// generates url format of return format.
    pub(crate) fn get_return_format_as_url(&self) -> String {
        self.return_format.generate_url_format()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_functionality_should_work() {
        let mut api_key = match ApiKey::from("abc".to_string()) {
            Ok(api_key) => api_key,
            Err(message) => {
                println!("{}", message.to_string());
                ApiKey("abc".to_string())
            },
        };

        if let Err(message) = api_key.change("new_key") {
            println!("{}", message.to_string());
        };
    }

    #[test]
    fn should_change_api_key() {
        let api_key = match ApiKey::from("abc".to_string()) {
            Ok(api_key) => api_key,
            Err(message) => {
                println!("{}", message.to_string());
                ApiKey("abc".to_string())
            },
        };

        let mut evds = Evds::from(api_key, ReturnFormat::Csv);

        if let Err(message) = evds.change_api_key("VALID_API_KEY") {
            println!("{}", message.to_string());
        }
    }

    #[test]
    fn evds_functionalities_should_work() {
        let api_key = match ApiKey::from("abc".to_string()) {
            Ok(api_key) => api_key,
            Err(message) => {
                println!("{}", message.to_string());
                ApiKey("abc".to_string())
            },
        };

        let mut evds = Evds::from(api_key, ReturnFormat::Csv);

        println!("{}", &evds.return_format.to_string());

        evds.change_return_format(ReturnFormat::Json);

        println!("{}", &evds.return_format.to_string());
        
        println!("\n\n{}\n{}\n",
        evds.return_format.generate_url_format(),
        evds.api_key.generate_url_format());

        evds.return_format = ReturnFormat::Csv;

        println!("\n\n{}\n{}\n",
        evds.return_format.generate_url_format(),
        evds.api_key.generate_url_format());

        evds.return_format = ReturnFormat::Xml;

        println!("\n\n{}\n{}\n",
        evds.return_format.generate_url_format(),
        evds.api_key.generate_url_format());
    }
}
