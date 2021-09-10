use crate::error::ReturnError;
use crate::evds_currency::{ExchangeType, CurrencyCode};
use crate::traits::ConvertingToRustEnum;


/// has required variables to crate `CurrencySeries`. 
#[derive(Debug)]
pub(crate) struct DataSeriesParts {
    pub exchange_type: ExchangeType,
    pub currency_code: CurrencyCode,
    pub ytl_mode: bool,
}

#[cfg(test)]
impl std::cmp::PartialEq for DataSeriesParts {
    fn eq(&self, other: &Self) -> bool {
        
        let exchange_type_comparison = 
            self.exchange_type.is_buying_type() == other.exchange_type.is_buying_type() &&
            self.exchange_type.is_selling_type() == other.exchange_type.is_selling_type() &&
            self.exchange_type.are_both_types() == other.exchange_type.are_both_types();

        let currency_code_comparison = self.currency_code.to_string() == other.currency_code.to_string();

        let ytl_mode_comparison = self.ytl_mode == other.ytl_mode;

        if exchange_type_comparison && currency_code_comparison && ytl_mode_comparison { return true; }

        false
    }
}


/// parses data series into currency unit, exchange type and ytl_mode. 
///
/// An instance for data series is `TP.DK.USD.S.YTL`.
pub(crate) fn parse_series(data_series: &str) -> Result<DataSeriesParts, ReturnError> {

    // Expected values.
    let min_series_length = 11;
    let max_series_length = 15;
    let min_dot_number = 3;
    let max_dot_number = 4;
    let min_separated_parts_number = 4;
    let max_separated_parts_number = 5;

    // Checking general validity.
    let mut out_of_range;


    let length = data_series.len();

    out_of_range = length < min_series_length || length > max_series_length;

    if out_of_range { return Err(ReturnError::InvalidSeries); }


    let mut dot_number: u8 = 0;
    for character in data_series.chars() {
        if character == '.' { dot_number += 1; }
    }

    out_of_range = dot_number < min_dot_number || dot_number > max_dot_number;

    if out_of_range { return Err(ReturnError::InvalidSeries); }

    
    let separated_data_series: Vec<&str> = data_series.split('.').collect();

    let vector_size = separated_data_series.len();

    out_of_range = vector_size < min_separated_parts_number || vector_size > max_separated_parts_number;

    if out_of_range { return Err(ReturnError::InvalidSeries); }
    

    // Making the required data series part via separated data series.
    let mut separated_parts = separated_data_series.iter();

    let mut ytl_mode = false;


    // ytl_mode occurs at only max separated parts number. 
    if vector_size == max_separated_parts_number {
        let ytl_mode_part = separated_parts.next_back().unwrap().to_ascii_lowercase();

        if ytl_mode_part != "ytl" { return Err(ReturnError::InvalidSeries); }

        ytl_mode = true;
    }


    let mut exchange_type = ExchangeType::new();

    let exchange_type_part = separated_parts.next_back().unwrap().to_ascii_lowercase();

    if &*exchange_type_part == "a" { exchange_type.select_buying_type(); }

    let currency_code_part = *separated_parts.next_back().unwrap();

    let currency_code = currency_code_part.convert();


    return Ok(DataSeriesParts { exchange_type, currency_code, ytl_mode });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_data_series() {
        // USD
        let mut data_series = "TP.DK.uSd.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Usd;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.USD.A.yTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Usd;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.USD.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Usd;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.USD.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Usd;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        
        // GBP
        let mut data_series = "TP.DK.GbP.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Gbp;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.GBP.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Gbp;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.GBP.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Gbp;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.GBP.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Gbp;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
        
        
        
        // PKR
        let mut data_series = "TP.DK.PKr.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Pkr;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.PKR.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Pkr;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.PKR.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Pkr;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.PKR.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Pkr;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);        
        
        

        // AUD
        let mut data_series = "TP.DK.aUd.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Aud;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.AUD.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Aud;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.AUD.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Aud;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.AUD.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Aud;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);      

        
        
        // DKK
        let mut data_series = "TP.DK.dKK.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Dkk;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.DKK.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Dkk;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.DKK.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Dkk;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.DKK.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Dkk;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);   
        
        

        // EUR
        let mut data_series = "TP.DK.EuR.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Eur;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.EUR.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Eur;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.EUR.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Eur;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.EUR.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Eur;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // CHF
        let mut data_series = "TP.DK.ChF.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Chf;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CHF.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Chf;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CHF.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Chf;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CHF.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Chf;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // SEK
        let mut data_series = "TP.DK.sEk.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Sek;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SEK.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Sek;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SEK.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Sek;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SEK.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Sek;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        
        
        // CAD
        let mut data_series = "TP.DK.caD.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Cad;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CAD.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Cad;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CAD.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Cad;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CAD.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Cad;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           


        // KWD
        let mut data_series = "TP.DK.KwD.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Kwd;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.KWD.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Kwd;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.KWD.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Kwd;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.KWD.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Kwd;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // NOK
        let mut data_series = "TP.DK.NOk.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Nok;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.NOK.A.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Nok;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.NOK.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Nok;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.NOK.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Nok;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // SAR
        let mut data_series = "TP.DK.sAr.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Sar;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SAR.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Sar;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SAR.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Sar;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.SAR.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Sar;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // JPY
        let mut data_series = "TP.DK.JPY.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Jpy;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.JPY.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Jpy;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.jPY.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Jpy;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.jPY.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Jpy;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // BGN
        let mut data_series = "TP.DK.BGN.S.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Bgn;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.BGN.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Bgn;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.bGN.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Bgn;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.BGN.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Bgn;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // RON
        let mut data_series = "TP.DK.ROn.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Ron;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RON.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Ron;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RON.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Ron;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RON.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Ron;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        
        
        // RUB
        let mut data_series = "TP.DK.RuB.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Rub;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RUB.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Rub;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RUB.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Rub;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.RUB.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Rub;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        
        
        // IRR
        let mut data_series = "TP.DK.irR.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Irr;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.IRR.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Irr;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.IRR.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Irr;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.IRR.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Irr;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        
        
        // QAR
        let mut data_series = "TP.DK.CnY.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Cny;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CNY.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Cny;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CNY.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Cny;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.CNY.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Cny;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
           
        

        // QAR
        let mut data_series = "TP.DK.QAR.s.YTL";

        let mut parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        let mut currency_code = CurrencyCode::Qar;
        let mut ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.QaR.a.YTL";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Qar;
        ytl_mode = true;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.QAR.A";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let mut exchange_type = ExchangeType::new();
        exchange_type.select_buying_type();
        currency_code = CurrencyCode::Qar;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);


        data_series = "TP.DK.QAR.S";

        parsing_result = parse_series(data_series);

        if let Err(return_error) = &parsing_result { println!("{}", return_error.to_string()); }
        
        let data_series_parts = parsing_result.unwrap();
    
        let exchange_type = ExchangeType::new();
        currency_code = CurrencyCode::Qar;
        ytl_mode = false;

        assert_eq!(DataSeriesParts { exchange_type, currency_code, ytl_mode }, data_series_parts);
    }
}
