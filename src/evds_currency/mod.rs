/// contains advanced currency operation requirements that are aggregation type, formula and data frequency included in
/// [`AdvancedProcesses`](struct@frequency_formulas::AdvancedProcesses).
pub(crate) mod frequency_formulas;

/// provides specific make request function for currency operations.
mod currency;


use self::frequency_formulas::*;

use crate::common;
use crate::error::ReturnError;
use crate::date::DatePreference;
use crate::traits::{self, MakingList, MakingUrlFormat, EnumSpecific, ConvertingToRustEnum};


/// contains exchange types, which are selling and buying, to configure currency request.
/// 
/// This structure is required for all *evds_currency* functions.
#[derive(Debug)]
pub(crate) struct ExchangeType {
    buying: bool,
    selling: bool,
}

impl ExchangeType {
    /// creates an exchange type variable with default type options.
    /// 
    /// Default type is selling.
    pub(crate) fn new() -> ExchangeType {
        ExchangeType {
            buying: false,
            selling: true,
        }
    }

    /// creates specified exchange type variable.
    pub(crate) fn from(buying: bool, selling: bool) -> ExchangeType {
        ExchangeType {
            buying,
            selling
        }
    }

    /// makes buying type true and selling type false.
    pub(crate) fn select_buying_type(&mut self) -> &mut Self {
        self.buying = true;
        self.selling = false;

        return self;
    }

    /// makes selling type true and buying type false.
    pub(crate) fn select_selling_type(&mut self) -> &mut Self {
        self.buying = false;
        self.selling = true;

        return self;
    }

    /// makes both type true.
    pub(crate) fn select_both_types(&mut self) -> &mut Self {
        self.buying = true;
        self.selling = true;

        return self;
    }

    pub(crate) fn is_selling_type(&self) -> bool {
        self.selling
    }

    pub(crate) fn is_buying_type(&self) -> bool {
        self.buying
    }

    pub(crate) fn are_both_types(&self) -> bool {
        if self.is_selling_type() && self.is_buying_type() { return true }
        
        false
    }
}

impl traits::MakingList for ExchangeType {
    fn make_required_list(&self) -> Vec<&str> {
        let mut exchange_type_list = Vec::new();

        if self.buying {
            exchange_type_list.push("A");
        }
        if self.selling {
            exchange_type_list.push("S");
        }

        exchange_type_list
    }
}


/// supplies currency code option to the functions making single currency request.
#[derive(Debug)]
pub(crate) enum CurrencyCode {
    Usd,
    Aud,
    Dkk,
    Eur,
    Gbp,
    Chf,
    Sek,
    Cad,
    Kwd,
    Nok,
    Sar,
    Jpy,
    Bgn,
    Ron,
    Rub,
    Irr,
    Cny,
    Pkr,
    Qar,
}

impl ToString for CurrencyCode {
    fn to_string(&self) -> String {
        match self {
            &Self::Usd => String::from("USD"),
            &Self::Aud => String::from("AUD"),
            &Self::Dkk => String::from("DKK"),
            &Self::Eur => String::from("EUR"),
            &Self::Gbp => String::from("GBP"),
            &Self::Chf => String::from("CHF"),
            &Self::Sek => String::from("SEK"),
            &Self::Cad => String::from("CAD"),
            &Self::Kwd => String::from("KWD"),
            &Self::Nok => String::from("NOK"),
            &Self::Sar => String::from("SAR"),
            &Self::Jpy => String::from("JPY"),
            &Self::Bgn => String::from("BGN"),
            &Self::Ron => String::from("RON"),
            &Self::Rub => String::from("RUB"),
            &Self::Irr => String::from("IRR"),
            &Self::Cny => String::from("CNY"),
            &Self::Pkr => String::from("PKR"),
            &Self::Qar => String::from("QAR"),
        }
    }
}

// This implementation is used for C FFI operations.
impl EnumSpecific for CurrencyCode {}

// This implementation is used for C FFI operations.
impl ConvertingToRustEnum<CurrencyCode> for &str {
    fn convert(&self) -> CurrencyCode {
        let lower_case_currency = &*self.to_ascii_lowercase();

        return match lower_case_currency {
            "usd" => CurrencyCode::Usd,
            "aud" => CurrencyCode::Aud,
            "dkk" => CurrencyCode::Dkk,
            "eur" => CurrencyCode::Eur,
            "gbp" => CurrencyCode::Gbp,
            "chf" => CurrencyCode::Chf,
            "sek" => CurrencyCode::Sek,
            "cad" => CurrencyCode::Cad,
            "kwd" => CurrencyCode::Kwd,
            "nok" => CurrencyCode::Nok,
            "sar" => CurrencyCode::Sar,
            "jpy" => CurrencyCode::Jpy,
            "bgn" => CurrencyCode::Bgn,
            "ron" => CurrencyCode::Ron,
            "rub" => CurrencyCode::Rub,
            "irr" => CurrencyCode::Irr,
            "cny" => CurrencyCode::Cny,
            "pkr" => CurrencyCode::Pkr,
            _     => CurrencyCode::Qar,
        }
    }
}


/// supplies currency codes to generate multiple currency series for 
/// [`MultipleCurrencySeries`](struct@MultipleCurrencySeries).
///
/// When a currency is decided to be used, its state should be true. 
///
/// The struct designed to select more than one currency code by making them true manually. It is also possible to 
/// select a currency code.
///
/// Default of CurrencyCodes makes all of the elements false.
///
/// # Usage
/// ```
///     let currency_codes_1 = CurrencyCodes {
///         usd: true,
///         aud: true,
///         pkr: true,
///         ..Default::default()
///     }
///     
///     let currency_codes_2 = CurrencyCodes {
///         gbp: true,
///         ..Default::default()
///     }
///
///     let currency_codes_3 = CurrencyCodes {
///         sar: true,
///         qar: true,
///         ..Default::default()
///     }
/// ```
pub(crate) struct CurrencyCodes {
    pub(crate) usd: bool,
    pub(crate) aud: bool,
    pub(crate) dkk: bool,
    pub(crate) eur: bool,
    pub(crate) gbp: bool,
    pub(crate) chf: bool,
    pub(crate) sek: bool,
    pub(crate) cad: bool,
    pub(crate) kwd: bool,
    pub(crate) nok: bool,
    pub(crate) sar: bool,
    pub(crate) jpy: bool,
    pub(crate) bgn: bool,
    pub(crate) ron: bool,
    pub(crate) rub: bool,
    pub(crate) irr: bool,
    pub(crate) cny: bool,
    pub(crate) pkr: bool,
    pub(crate) qar: bool,
}

impl Default for CurrencyCodes {
    fn default() -> CurrencyCodes {
        CurrencyCodes {
            usd: false,
            aud: false,
            dkk: false,
            eur: false,
            gbp: false,
            chf: false,
            sek: false,
            cad: false,
            kwd: false,
            nok: false,
            sar: false,
            jpy: false,
            bgn: false,
            ron: false,
            rub: false,
            irr: false,
            cny: false,
            pkr: false,
            qar: false,
        }
    }
}

impl CurrencyCodes {
    /// "usd" is used as default currency code. 
    pub(crate) fn new() -> CurrencyCodes {
        CurrencyCodes {
            usd: true,
            ..Default::default()
        }
    }

    /// "usd" is used as default currency code.
    pub(crate) fn reset(&mut self) {
        *self = CurrencyCodes{
            usd: true,
            ..Default::default()
        };
    }

    /// makes all currency codes ON.
    pub(crate) fn include_all(&mut self) {
        self.usd = true;
        self.aud = true;
        self.dkk = true;
        self.eur = true;
        self.gbp = true;
        self.chf = true;
        self.sek = true;
        self.cad = true;
        self.kwd = true;
        self.nok = true;
        self.sar = true;
        self.jpy = true;
        self.bgn = true;
        self.ron = true;
        self.rub = true;
        self.irr = true;
        self.cny = true;
        self.pkr = true;
        self.qar = true;
    }

    /// makes all currency codes OFF.
    pub(crate) fn exclude_all(&mut self) {
        *self = CurrencyCodes::default();
    }

    /// checks the situation all currency codes are OFF.
    pub(crate) fn is_all_excluded(&self) -> bool {
        if self.usd { return false }
        if self.aud { return false }
        if self.dkk { return false }
        if self.eur { return false }
        if self.gbp { return false }
        if self.chf { return false }
        if self.sek { return false }
        if self.cad { return false }
        if self.kwd { return false }
        if self.nok { return false }
        if self.sar { return false }
        if self.jpy { return false }
        if self.bgn { return false }
        if self.ron { return false }
        if self.rub { return false }
        if self.irr { return false }
        if self.cny { return false }
        if self.pkr { return false }
        if self.qar { return false }
        
        true
    }
}

impl traits::MakingList for CurrencyCodes {
    /// makes a list of used currency codes.
    fn make_required_list(&self) -> Vec<&str> {
        let mut currency_codes = Vec::new();
        
        if self.usd { currency_codes.push("USD"); }
        if self.aud { currency_codes.push("AUD"); }
        if self.dkk { currency_codes.push("DKK"); }
        if self.eur { currency_codes.push("EUR"); }
        if self.gbp { currency_codes.push("GBP"); }
        if self.chf { currency_codes.push("CHF"); }
        if self.sek { currency_codes.push("SEK"); }
        if self.cad { currency_codes.push("CAD"); }
        if self.kwd { currency_codes.push("KWD"); }
        if self.nok { currency_codes.push("NOK"); }
        if self.sar { currency_codes.push("SAR"); }
        if self.jpy { currency_codes.push("JPY"); }
        if self.bgn { currency_codes.push("BGN"); }
        if self.ron { currency_codes.push("RON"); }
        if self.rub { currency_codes.push("RUB"); }
        if self.irr { currency_codes.push("IRR"); }
        if self.cny { currency_codes.push("CNY"); }
        if self.pkr { currency_codes.push("PKR"); }
        if self.qar { currency_codes.push("QAR"); }

        currency_codes
    }
}


/// supplies reliable and well structured required details about currency and date/s to the functions making single 
/// currency operations such as [`get_data`](fn@CurrencySeries::get_data) and 
/// [`get_advanced_data`](fn@CurrencySeries::get_advanced_data).
///
/// This struct accepts both Single and Multiple date options.
///
/// It is recommended CurrencySeries variable to be created via [`from`](fn@CurrencySeries::from).
///
/// *Use of this struct and its implemented functions seems complicated, however it is safe and makes some required 
/// error prone operations automatically without any problem.*
pub(crate) struct CurrencySeries {
    pub(crate) ytl_mode: bool,
    pub(crate) exchange_type: ExchangeType,
    pub(crate) currency_code: CurrencyCode,
    pub(crate) date_preference: DatePreference,
}

impl CurrencySeries {
    /// generates single series or dual series with selling and buying with given data.
    fn generate_series_as_url_format(&self) -> Result<String, ReturnError> {
        let exchange_types = self.exchange_type.make_required_list();
        
        let series_format: String;  

        if exchange_types.is_empty() {
            return Err(ReturnError::EmptyExchangeType);
        }

        if exchange_types.len() == 2 {

            series_format =
            <Self as MakingUrlFormat>::generate_two_combined_currencies_format(
                &<Self as MakingUrlFormat>::generate_currency_format_for_combination(
                    &self.currency_code.to_string(),
                    exchange_types[0],
                    self.ytl_mode
                ),
                &<Self as MakingUrlFormat>::generate_currency_format_for_combination(
                    &self.currency_code.to_string(), 
                    exchange_types[1],
                    self.ytl_mode
                )
            );
        }
        else {
            series_format = 
            <Self as MakingUrlFormat>::generate_currency_format(
                &self.currency_code.to_string(), 
                exchange_types[0], self.ytl_mode
            );
        }

        Ok(series_format)
    }


    /// creates currency series with detailed information.
    /// 
    /// # Example
    /// ```
    ///     use tcmb_evds_c::date::{Date, DatePreference};        
    ///     use tcmb_evds_c::evds_currency::{ExchangeType, CurrencyCode, CurrencySeries};
    /// 
    /// 
    ///     let exchange_type = ExchangeType::new();
    ///
    ///     let currency_code = CurrencyCode::Qar;
    ///
    ///     let date_result = Date::from("13-12-2011");
    ///     let date = 
    ///         if let Ok(date) = date_result { date } 
    ///         else { return };     
    ///     let date_preference = DatePreference::Single(date);
    ///     
    ///     // Ytl mode adds "YTL" to currency series, when it is true.
    ///     let ytl_mode = true;
    ///    
    ///    
    ///     let currency_series = CurrencySeries::from(exchange_type, currency_code, date_preference, ytl_mode);
    /// ```
    pub(crate) fn from(
        exchange_type: ExchangeType, 
        currency_code: CurrencyCode, 
        date_preference: DatePreference,
        ytl_mode: bool
    ) -> CurrencySeries {
        CurrencySeries {
            ytl_mode,
            exchange_type,
            currency_code,
            date_preference,
        }
    }


    /// returns data about just one currency.
    ///
    /// Single date or multiple dates can be used for this function.
    ///
    /// This function is used as a method of [`CurrencySeries`](struct@CurrencySeries) because of decreasing amount of
    /// function parameters user entering.
    ///
    /// # Error
    ///  
    /// This function returns error if internet connection is lost.
    ///
    /// # Example
    ///
    /// Follow [`Evds`](crate::common::Evds) for detailed implementation of *evds*.
    ///
    /// ```
    /// #   use tcmb_evds_c::date::{Date, DatePreference};        
    /// #   use tcmb_evds_c::evds_currency::{ExchangeType, CurrencyCode};
    /// #   use tcmb_evds_c::common::{ApiKey, ReturnFormat, Evds};
    /// #   use tcmb_evds_c::evds_currency::CurrencySeries;
    /// #
    /// #   let exchange_type = ExchangeType::new();
    /// #
    /// #   let currency_code = CurrencyCode::Qar;
    /// #
    /// #   let date_result = Date::from("13-12-2011");
    /// #   let date = 
    /// #       if let Ok(date) = date_result { date } 
    /// #       else { return };     
    /// #   let date_preference = DatePreference::Single(date);
    /// #   
    /// #   // Ytl mode adds "YTL" to currency series, when it is true.
    /// #   let ytl_mode = true;
    /// #  
    /// #   let currency_series = CurrencySeries::from(exchange_type, currency_code, date_preference, ytl_mode);
    /// #
    /// #   let api_key = 
    /// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
    /// #       else { return }; 
    /// #   let evds = Evds::from(api_key, ReturnFormat::Json);
    /// #
    ///     // requesting currency data.
    ///     let result = currency_series.get_data(&evds);
    ///
    ///
    ///     let currency_data = match result {
    ///         Ok(response) => response,
    ///         Err(error) => {
    ///             println!("{}", error.to_string());
    ///             return
    ///         }
    ///     };
    /// ```
    pub(crate) fn get_data(&self, evds: &common::Evds) -> Result<String, ReturnError> {
        
        let url_root = "https://evds2.tcmb.gov.tr/service/evds/";

        let series_format = self.generate_series_as_url_format()?;

        let url = format!(
            "{}{}&{}&{}&{}", 
            url_root, 
            series_format, 
            self.date_preference.generate_url_format(), 
            evds.get_return_format_as_url(), 
            evds.get_api_key_as_url());

        currency::make_request(&url)
    }


    /// returns data about just one currency with frequency formulas.
    ///
    /// Single date or multiple dates can be used for this function.
    ///
    /// This function is used as a method of [`CurrencySeries`](struct@CurrencySeries) because of decreasing amount of
    /// function parameters user entering.
    ///
    /// # Error
    ///  
    /// This function returns error if internet connection is lost.
    ///
    /// # Example
    ///
    /// Follow [`Evds`](crate::common::Evds) and 
    /// [`AdvancedProcesses`](crate::evds_currency::frequency_formulas::AdvancedProcesses) for detailed implementation 
    /// of *evds* and *advanced_processes*.
    ///
    /// ```
    /// #   use tcmb_evds_c::date::{Date, DatePreference};        
    /// #   use tcmb_evds_c::evds_currency::{ExchangeType, CurrencyCode};
    /// #   use tcmb_evds_c::common::{ApiKey, ReturnFormat, Evds};
    /// #   use tcmb_evds_c::evds_currency::{CurrencySeries, frequency_formulas::*};
    /// #
    /// #   let exchange_type = ExchangeType::new();
    /// #
    /// #   let currency_code = CurrencyCode::Qar;
    /// #
    /// #   let date_result = Date::from("13-12-2011");
    /// #   let date = 
    /// #       if let Ok(date) = date_result { date } 
    /// #       else { return };     
    /// #   let date_preference = DatePreference::Single(date);
    /// #   
    /// #   // Ytl mode adds "YTL" to currency series, when it is true.
    /// #   let ytl_mode = true;
    /// #  
    /// #   let currency_series = CurrencySeries::from(exchange_type, currency_code, date_preference, ytl_mode);
    /// #
    /// #   let api_key = 
    /// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
    /// #       else { return }; 
    /// #   let evds = Evds::from(api_key, ReturnFormat::Json);
    /// #
    /// #   let aggregation_type = AggregationType::Average;
    /// #   let formula = Formula::Level;
    /// #   let data_frequency = DataFrequency::Monthly;
    /// #
    /// #   let advanced_processes = AdvancedProcesses::from(aggregation_type, formula, data_frequency);
    /// #
    ///     // requesting currency data with frequency formulas.
    ///     let result = currency_series.get_advanced_data(&evds, &advanced_processes);
    /// 
    /// 
    ///     let advanced_currency_data = match result {
    ///         Ok(response) => response,
    ///         Err(error) => {
    ///             println!("{}", error.to_string());
    ///             return
    ///         }
    ///     };
    /// ```
    pub(crate) fn get_advanced_data(
        &self, 
        evds: &common::Evds, 
        advanced_processes: &AdvancedProcesses
    ) -> Result<String, ReturnError> {
        
        let url_root = "https://evds2.tcmb.gov.tr/service/evds/";

        if self.exchange_type.are_both_types() {
            return Err(ReturnError::SingleExchangeTypeExpected)
        }

        let series_format = self.generate_series_as_url_format()?;

        let url = format!(
            "{}{}&{}&{}&{}&{}&{}&{}", 
            url_root, series_format, 
            self.date_preference.generate_url_format(), 
            evds.get_return_format_as_url(), 
            evds.get_api_key_as_url(), 
            advanced_processes.get_aggregation_type_as_url_format(), 
            advanced_processes.get_formula_as_url_format(), 
            advanced_processes.get_data_frequency_as_url_format()
        );
    
        currency::make_request(&url)
    }
}

impl traits::MakingUrlFormat for CurrencySeries {}


/// supplies reliable and well structured required details about multiple currencies and date/s to the functions 
/// making multiple currency operations such as 
/// [`get_multiple_data`](crate::evds_currency::MultipleCurrencySeries::get_multiple_data).
///
/// This struct accepts both Single and Multiple date options.
///
/// *Use of this struct and its implemented functions seems complicated, however it is safe and makes some required 
/// error prone operations automatically without any problem.*
pub(crate) struct MultipleCurrencySeries {
    pub(crate) ytl_mode: bool,
    pub(crate) exchange_type: ExchangeType,
    pub(crate) currency_codes: CurrencyCodes,
    pub(crate) date_preference: DatePreference,
}

impl MultipleCurrencySeries {
    fn generate_multiple_series_as_url_format(&self) -> Result<String, ReturnError> {
        let currency_codes = self.currency_codes.make_required_list();
        let exchange_types = self.exchange_type.make_required_list();

        let series_format: String;

        if currency_codes.is_empty() {
            return Err(ReturnError::EmptyCurrencyCodes);
        }

        if exchange_types.is_empty() {
            return Err(ReturnError::EmptyExchangeType);
        }
        
        if exchange_types.len() == 2 {
            series_format = <Self as MakingUrlFormat>::generate_two_combined_currencies_format(
                &<Self as MakingUrlFormat>::generate_multiple_currency_format_for_combination(
                    self.currency_codes.make_required_list(), 
                    exchange_types[0], 
                    self.ytl_mode
                ), 
                &<Self as MakingUrlFormat>::generate_multiple_currency_format_for_combination(
                    self.currency_codes.make_required_list(), 
                    exchange_types[1], 
                    self.ytl_mode
                )
            );
        }
        else {
            series_format = <Self as MakingUrlFormat>::generate_multiple_currency_format(
                self.currency_codes.make_required_list(), 
                exchange_types[0], 
                self.ytl_mode
            );
        }

        Ok(series_format)
    }
    
    /// creates multiple currency series with detailed information.
    /// 
    /// # Example
    /// ```
    ///     use tcmb_evds_c::date::{DateRange, DatePreference};        
    ///     use tcmb_evds_c::evds_currency::{ExchangeType, CurrencyCodes, MultipleCurrencySeries};
    /// 
    /// 
    ///     let exchange_type = ExchangeType::new();
    ///
    ///     let mut currency_codes = CurrencyCodes { 
    ///         usd: true,
    ///         aud: true,
    ///         ..Default::default()
    ///     };
    ///
    ///     let date_range_result = DateRange::from("13-12-2011", "12-12-2012");
    ///
    ///     let date_range = 
    ///         if let Ok(dates) = date_range_result { dates } 
    ///         else { return };     
    ///
    ///     let date_preference = DatePreference::Multiple(date_range);
    ///     
    ///     // Ytl mode adds "YTL" to currency series, when it is true.
    ///     let ytl_mode = false;
    /// 
    ///    
    ///     let currency_series = 
    ///         MultipleCurrencySeries::from(
    ///             exchange_type, 
    ///             currency_codes, 
    ///             date_preference, 
    ///             ytl_mode
    ///         );
    /// ```
    pub(crate) fn from(
        exchange_type: ExchangeType, 
        currency_codes: CurrencyCodes, 
        date_preference: DatePreference,
        ytl_mode: bool,
    ) -> MultipleCurrencySeries {
        MultipleCurrencySeries {
            ytl_mode,
            exchange_type,
            currency_codes,
            date_preference,
        }   
    }

    /// returns data about more than one currency.
    ///
    /// Single date or multiple dates can be used for this function.
    ///
    /// This function is used as a method of [`CurrencySeries`](struct@CurrencySeries) because of decreasing amount of
    /// function parameters user entering.
    ///
    /// # Error
    ///  
    /// This function returns error if internet connection is lost.
    ///
    /// # Example
    ///
    /// Follow [`Evds`](crate::common::Evds) for detailed implementation of *evds*.
    ///
    /// ```
    /// #   use tcmb_evds_c::date::{Date, DateRange, DatePreference};        
    /// #   use tcmb_evds_c::evds_currency::{ExchangeType, CurrencyCodes, MultipleCurrencySeries};
    /// #   use tcmb_evds_c::common::{ApiKey, ReturnFormat, Evds};    
    /// #
    /// #   let exchange_type = ExchangeType::new();
    /// #
    /// #   let mut currency_codes = CurrencyCodes::new();
    /// #   currency_codes.aud = true;
    /// #
    /// #   let date_range_result = DateRange::from("13-12-2011", "12-12-2012");
    /// #   let date_range = 
    /// #       if let Ok(dates) = date_range_result { dates }
    /// #       else { return };     
    /// #   let date_preference = DatePreference::Multiple(date_range);
    /// #   
    /// #   // Ytl mode adds "YTL" to currency series, when it is true.
    /// #   let ytl_mode = false;
    /// #  
    /// #   let currency_series = 
    /// #       MultipleCurrencySeries::from(
    /// #           exchange_type, 
    /// #           currency_codes, 
    /// #           date_preference, 
    /// #           ytl_mode
    /// #       );
    /// #
    /// #   let api_key = 
    /// #       if let Ok(api_key) = ApiKey::from("users_api_key") { api_key } 
    /// #       else { return }; 
    /// #   let evds = Evds::from(api_key, ReturnFormat::Json);
    /// #
    ///     // requesting more than one currency data. 
    ///     let result = currency_series.get_multiple_data(&evds);
    /// 
    ///
    ///     let multiple_currency_data = match result {
    ///         Ok(response) => response,
    ///         Err(error) => {
    ///             println!("{}", error.to_string());
    ///             return
    ///         }
    ///     };
    /// ```
    pub(crate) fn get_multiple_data(&self, evds: &common::Evds) -> Result<String, ReturnError> {
        
        let url_root = "https://evds2.tcmb.gov.tr/service/evds/";

        let series_format = self.generate_multiple_series_as_url_format()?;

        let url = format!(
            "{}{}&{}&{}&{}", 
            url_root, series_format,
            self.date_preference.generate_url_format(),
            evds.get_return_format_as_url(), 
            evds.get_api_key_as_url()
        );

        currency::make_request(&url)
    }
}

impl traits::MakingUrlFormat for MultipleCurrencySeries {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_make_default() {

        let mut currency_codes = CurrencyCodes {
            aud: true,
            qar: true,
            usd: true,
            kwd: true,
            ..Default::default()
        };

        currency_codes.sar = true;

        assert!(!currency_codes.is_all_excluded());

        let currency_codes_list = currency_codes.make_required_list();

        for code in currency_codes_list {
            println!("{}", &code);
        }
    }
}
