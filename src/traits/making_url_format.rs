pub(crate) trait MakingUrlFormat {
    /// is overridden in actual usage with applicable data structures.
    fn generate_url_format(&self) -> String {
        "nothing".to_string()
    }

    /// generates suitable currency format to be used in the latest version of the url.
    fn generate_currency_format(currency: &str, exchange_type: &str, ytl_mode: bool) -> String {
        if ytl_mode {
            return format!("series=TP.DK.{}.{}.YTL", currency, exchange_type);
        }
        
        format!("series=TP.DK.{}.{}", currency, exchange_type)
    }

    /// generates suitable multiple currency series url format to be used in the latest version of the url.
    fn generate_multiple_currency_format(currencies: Vec<&str>, exchange_type: &str, ytl_mode: bool) -> String {
        let mut buffer = String::from("series=");

        for currency in currencies {
            if ytl_mode {
                buffer.push_str(&format!("TP.DK.{}.{}.YTL-", currency, exchange_type));
             
                continue;
            }
            buffer.push_str(&format!("TP.DK.{}.{}-", currency, exchange_type));
        }

        buffer.pop();

        buffer
    }

    /// generates url format of a given buying/selling type currency to be combined with its corresponding 
    /// selling/buying version later.
    ///
    /// Ytl mode given as parameter configures return value format of currency series.
    ///
    /// Printout examples:
    /// ```
    ///     1. Ytl Mode On : TP.DK.USA.A.YTL 
    ///     2. Ytl Mode Off: TP.DK.USA.A
    ///     
    /// ```
    /// Later couples:
    /// ```
    ///     1. -> TP.DK.USA.S.YTL
    ///     2. -> TP.DK.USA.S
    /// 
    ///     ("->" means "will be combined with")
    /// ```
    fn generate_currency_format_for_combination(currency: &str, exchange_type: &str, ytl_mode: bool) -> String {
        if ytl_mode {
            return format!("TP.DK.{}.{}.YTL", currency, exchange_type);
        }
        
        format!("TP.DK.{}.{}", currency, exchange_type)
    }
  
    /// generates url format of given buying/selling type currencies to be combined with their corresponding 
    /// selling/buying versions later.
    ///
    /// Ytl mode given as parameter configures return value format of currency series.
    ///
    /// Printout examples:
    /// ```
    ///     1. Ytl Mode On : TP.DK.USD.A.YTL-TP.DK.EUR.A.YTL-TP.DK.GBP.A.YTL
    ///     2. Ytl Mode Off: TP.DK.USD.S-TP.DK.AUD.S-TP.DK.GBP.S
    ///     
    /// ```
    /// Later couples:
    /// ```
    ///     1. -> TP.DK.USA.S.YTL
    ///     2. -> TP.DK.USA.S
    /// 
    ///     ("->" means "will be combined with")
    /// ```
    fn generate_multiple_currency_format_for_combination(
        currencies: Vec<&str>, 
        exchange_type: &str, 
        ytl_mode: bool
    ) -> String {
        let mut buffer = String::new();

        for currency in currencies {
            if ytl_mode {
                buffer.push_str(&format!("TP.DK.{}.{}.YTL-", currency, exchange_type));

                continue;
            }
            buffer.push_str(&format!("TP.DK.{}.{}-", currency, exchange_type));
        }

        buffer.pop();

        buffer
    }

    /// generates url format combination of given two currency series.  
    ///
    /// This function should be used when double exchange type is provided.
    /// 
    /// Buying and Selling Types Output Examples: 
    /// ```
    ///     Buying: TP.DK.A.JPY, Selling: TP.DK.S.JPY
    /// 
    ///     Combined_series: TP.DK.A.JPY-TP.DK.S.JPY
    /// ```
    fn generate_two_combined_currencies_format(former_currency: &str, latter_currency: &str) -> String {
        format!("series={}-{}", former_currency, latter_currency)
    }
}
