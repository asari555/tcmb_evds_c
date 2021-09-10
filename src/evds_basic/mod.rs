/// provides specific make request function for basic operations.
mod basic;

use crate::date;
use crate::common;
use crate::error::ReturnError;
use crate::traits::MakingUrlFormat;

/// returns data about requested data series.
///
/// Data should be detached from the result to have information about required data series.
///
/// [`Date`](struct@crate::date::Date) and [`DateRange`](struct@crate::date::DateRange) are both acceptable
/// for this function.
/// 
/// *User is responsible to supply correct and valid* **data series** *argument for this function.*
///
/// # Error
///
/// This function returns an error if some of given parameters is empty, the internet connection is poor or/and
/// the format of the request is invalid or/and incorrect. 
///
/// # Example
///
/// Follow [`Evds`](crate::common::Evds) for full and detailed implementation of **evds** argument.
///
/// ```
/// #   use tcmb_evds::date::{Date, DatePreference};     
/// #   use tcmb_evds::common::{Evds, ApiKey, ReturnFormat};     
///     use tcmb_evds::evds_basic;
/// 
/// 
///     // declaration and assignment of required argument.
///     // another possible data series = "TP.DK.USD.A-TP.DK.USD.S-TP.DK.GBP.A-TP.DK.GBP.S" 
///     let data_series = "TP.DK.USD.A";
/// #
/// #   let date_result = Date::from("13-12-2011");
/// #   let date = 
/// #       if let Ok(date) = date_result { date } 
/// #       else { return };     
/// #   let date_preference = DatePreference::Single(date);
/// #   
/// #   let api_key = 
/// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
/// #       else { return }; 
/// #   
/// #   let evds = Evds::from(api_key, ReturnFormat::Xml);
/// 
///
///     // requesting data section.
///     let result = evds_basic::get_data(data_series, &date_preference, &evds);
///
///     
///     // error handling or getting the data.
///     let data = match result {
///         Err(error) => { 
///             println!("{}", error.to_string());
///             return;
///         },
///         Ok(data) => data,     
///     };
/// ```
pub(crate) fn get_data<'a>(
    data_series: &str, 
    date_preference: &date::DatePreference, 
    evds: &common::Evds,
) -> Result<String, ReturnError> {

    let dates_as_url = date_preference.generate_url_format();
    let return_format_as_url = evds.get_return_format_as_url();
    let api_key_as_url = evds.get_api_key_as_url();

    basic::check_emptiness(data_series)?;

    let url = 
    format!(
        "https://evds2.tcmb.gov.tr/service/evds/series={}&{}&{}&{}", 
        data_series, 
        dates_as_url,
        return_format_as_url, 
        api_key_as_url,
    );
    
    basic::make_request(&url, basic::Function::OneOfOtherFunctions)
}

/// returns requested data group.
///
/// Data should be detached from the result to have data group information.
///
/// This function expects only *single* data group.
///
/// [`Date`](struct@crate::date::Date) and [`DateRange`](struct@crate::date::DateRange) are both acceptable
/// for this function.
/// 
/// *User is responsible to supply correct and valid* **data group** *argument for this function.*
///
/// # Error
///
/// This function returns an error if some of given parameters is empty, the internet connection is poor or/and
/// the format of the request is invalid or/and incorrect. 
///
/// # Example
///
/// Follow [`DatePreference`](crate::date::DatePreference) and [`Evds`](crate::common::Evds) for full and detailed 
/// implementation of **date_preference** and **evds** arguments respectively.
///
/// ```
/// #   use tcmb_evds::date::{DateRange, DatePreference};     
/// #   use tcmb_evds::common::{Evds, ApiKey, ReturnFormat};     
///     use tcmb_evds::evds_basic;
/// 
/// 
///     // declaration and assignment of required argument.
///     let data_group = "bie_yssk";
/// #
/// #   let date_range_result = DateRange::from("13-12-2011", "12-11-2015");
/// #   let date_range = 
/// #       if let Ok(date_range) = date_range_result { date_range } 
/// #       else { return };     
/// #   let date_preference = DatePreference::Multiple(date_range);
/// #    
/// #   let api_key = 
/// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
/// #       else { return }; 
/// #   
/// #   let evds = Evds::from(api_key, ReturnFormat::Xml);
/// #
///
///
///     // requesting data section.
///     let result = evds_basic::get_data_group(data_group, &date_preference, &evds);
///
///     
///     // error handling or getting the data.
///     let data = match result {
///         Err(error) => { 
///             println!("{}", error.to_string());
///             return;
///         },
///         Ok(data) => data,     
///     };
/// ```
pub(crate) fn get_data_group(
    data_group: &str, 
    date_preference: &date::DatePreference, 
    evds: &common::Evds,
) -> Result<String, ReturnError> {

    let dates_as_url = date_preference.generate_url_format();
    let return_format_as_url = evds.get_return_format_as_url();
    let api_key_as_url = evds.get_api_key_as_url();

    basic::check_emptiness(data_group)?;

    let url = 
    format!(
        "https://evds2.tcmb.gov.tr/service/evds/datagroup={}&{}&{}&{}", 
        data_group, 
        dates_as_url,
        return_format_as_url, 
        api_key_as_url,
    );

    basic::make_request(&url, basic::Function::GetDataGroup)
}

/// returns all requested categories of EVDS.
///
/// Data should be detached from the result to have categories.
///
/// # Error
///
/// This function returns an error if some of given parameters is empty, the internet connection is poor or/and
/// the format of the request is invalid or/and incorrect. 
///
/// # Example
///
/// Follow [`Evds`](crate::common::Evds) for full and detailed implementation of **evds** argument.
///
/// ```
/// #   use tcmb_evds::common::{Evds, ApiKey, ReturnFormat};     
///     use tcmb_evds::evds_basic;
/// 
/// 
/// #   let api_key = 
/// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
/// #       else { return }; 
/// #   
/// #   let evds = Evds::from(api_key, ReturnFormat::Xml);
/// #
///     // requesting data section.
///     let result = evds_basic::get_categories(&evds);
///     
///     
///     // error handling or getting the data.
///     let data = match result {
///         Err(error) => { 
///             println!("{}", error.to_string());
///             return;
///         },
///         Ok(data) => data,     
///     };
/// ```
pub(crate) fn get_categories(evds: &common::Evds) -> Result<String, ReturnError> {

    let return_format_as_url = evds.get_return_format_as_url();
    let api_key_as_url = evds.get_api_key_as_url();

    let url = 
    format!{
        "https://evds2.tcmb.gov.tr/service/evds/categories/{}&{}",
        api_key_as_url,
        return_format_as_url,
    };

    basic::make_request(&url, basic::Function::OneOfOtherFunctions)
}

/// returns required data groups.
///
/// Data should be detached from the result to have information about data groups.
/// 
/// It is because code parameter can be a string parameter that this parameter is required as string slice.
///
/// *Users are responsible to supply correct and valid* **mode** *and* **code** *arguments for this function.*
///
/// # Error
///
/// This function returns an error if some of given parameters is empty, the internet connection is poor or/and
/// the format of the request is invalid or/and incorrect. 
///
/// # Example
///
/// Follow [`Evds`](crate::common::Evds) for full and detailed implementation of **evds** argument.
///
/// ```
/// #   use tcmb_evds::common::{Evds, ApiKey, ReturnFormat};     
///     use tcmb_evds::evds_basic;
/// 
///     
/// #   let api_key = 
/// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
/// #       else { return }; 
/// #   
/// #   let evds = Evds::from(api_key, ReturnFormat::Xml);
/// #
///     // declaration and assignment of the required arguments.
///     let mode: u32 = 1;
///     let code: &str = "bie_yssk";
///
///
///     // requesting data section.
///     let result = evds_basic::get_advanced_data_group(mode, code, &evds);
///     
///     
///     // error handling or getting the data.
///     let data = match result {
///         Err(error) => { 
///             println!("{}", error.to_string());
///             return;
///         },
///         Ok(data) => data,     
///     };
/// ```
pub(crate) fn get_advanced_data_group(
    mode: u32, 
    code: &str, 
    evds: &common::Evds
) -> Result<String, ReturnError> {

    let return_format_as_url = evds.get_return_format_as_url();
    let api_key_as_url = evds.get_api_key_as_url();
    
    basic::check_emptiness(code)?;
    
    let url = 
    format!(
        "https://evds2.tcmb.gov.tr/service/evds/datagroups/{}&mode={}&code={}&{}", 
        api_key_as_url,
        mode, 
        code, 
        return_format_as_url, 
    );

    basic::make_request(&url, basic::Function::OneOfOtherFunctions)
}

/// returns all usable series list.
///
/// Data should be detached from the result to have the list of series.
/// 
/// It is because code parameter can be a string parameter that this parameter is required as string slice.
///
/// *User is responsible to supply correct and valid* **code** *argument for this function.*
///
/// # Error
///
/// This function returns an error if some of given parameters is empty, the internet connection is poor or/and
/// the format of the request is invalid or/and incorrect. 
///
/// # Example
///
/// Follow [`Evds`](crate::common::Evds) for full and detailed implementation of **evds** argument.
///
/// ```
/// #   use tcmb_evds::common::{Evds, ApiKey, ReturnFormat};     
///     use tcmb_evds::evds_basic;
/// 
///     
/// #   let api_key = 
/// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
/// #       else { return }; 
/// #   
/// #   let evds = Evds::from(api_key, ReturnFormat::Csv);
/// #
///     // required data declaration and assignment.
///     let code: &str = "bie_yssk";
///
///
///     // requesting data section.
///     let result = evds_basic::get_series_list(code, &evds);
///     
///     
///     // error handling or getting the data.
///     let data = match result {
///         Err(error) => { 
///             println!("{}", error.to_string());
///             return;
///         },
///         Ok(data) => data,     
///     };
/// ```
pub(crate) fn get_series_list(
    code: &str, 
    evds: &common::Evds
) -> Result<String, ReturnError> {
    
    if code.is_empty() { return Err(ReturnError::EmptyParameter); }

    let return_format_as_url = evds.get_return_format_as_url();
    let api_key_as_url = evds.get_api_key_as_url();

    let url = 
    format!(
        "https://evds2.tcmb.gov.tr/service/evds/serieList/{}&{}&code={}",
        api_key_as_url,
        return_format_as_url, 
        code, 
    );

    basic::make_request(&url, basic::Function::GetSeriesList)
}
