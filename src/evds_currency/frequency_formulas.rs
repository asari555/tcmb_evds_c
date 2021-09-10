use super::*;

/// provides aggregation type options to create an element of frequency formulas.
///
/// This enum is used in [`AdvancedProcesses`](crate::evds_currency::frequency_formulas::AdvancedProcesses) for 
/// [`get_advanced_data`](crate::evds_currency::CurrencySeries::get_advanced_data) function.
pub enum AggregationType {
    Average,
    Minimum,
    Maximum,
    Beginning,
    End,
    Cumulative,
}

impl ToString for AggregationType {
    fn to_string(&self) -> String {
        match self {
            &Self::Average => String::from("avg"),
            &Self::Minimum => String::from("min"),
            &Self::Maximum => String::from("max"),
            &Self::Beginning => String::from("first"),
            &Self::End => String::from("last"),
            &Self::Cumulative => String::from("sum"),
        }
    }
}

impl traits::MakingUrlFormat for AggregationType {
    fn generate_url_format(&self) -> String {
        format!("aggregationTypes={}", self.to_string())
    }
}


/// provides formula options to create an element of frequency formulas.
///
/// This enum is used in [`AdvancedProcesses`](crate::evds_currency::frequency_formulas::AdvancedProcesses) for 
/// [`get_advanced_data`](crate::evds_currency::CurrencySeries::get_advanced_data) function.
pub enum Formula {
    Level, 
    PercentageChange, 
    Difference, 
    YearToYearPercentChange, 
    YearToYearDifferences,
    PercentageChangeByEndOfPreviousYear, 
    DifferenceByEndOfPreviousYear,
    MovingAverage, 
    MovingSum,
}

impl ToString for Formula {
    fn to_string(&self) -> String {
        match self {
            &Self::Level => "0".to_string(),
            &Self::PercentageChange => "1".to_string(),
            &Self::Difference => "2".to_string(),
            &Self::YearToYearPercentChange => "3".to_string(),
            &Self::YearToYearDifferences => "4".to_string(),
            &Self::PercentageChangeByEndOfPreviousYear => "5".to_string(),
            &Self::DifferenceByEndOfPreviousYear => "6".to_string(),
            &Self::MovingAverage => "7".to_string(),
            &Self::MovingSum => "8".to_string(),
        }
    }
} 

impl traits::MakingUrlFormat for Formula {
    fn generate_url_format(&self) -> String {
        format!("formulas={}", self.to_string())
    }
}


/// provides data frequency options to create an element of frequency formulas.
///
/// This struct is used for [`AdvancedProcesses`](crate::evds_currency::frequency_formulas::AdvancedProcesses) for 
/// [`get_advanced_data`](crate::evds_currency::CurrencySeries::get_advanced_data) function.
pub enum DataFrequency {
    Daily, 
    Business, 
    WeeklyFriday,
    TwiceMonthly, 
    Monthly, 
    Quarterly,
    SemiAnnual, 
    Annual,
}

impl ToString for DataFrequency {
    fn to_string(&self) -> String {
        match self {
            &Self::Daily => "1".to_string(),
            &Self::Business => "2".to_string(),
            &Self::WeeklyFriday => "3".to_string(),
            &Self::TwiceMonthly => "4".to_string(),
            &Self::Monthly => "5".to_string(),
            &Self::Quarterly => "6".to_string(),
            &Self::SemiAnnual => "7".to_string(),
            &Self::Annual => "8".to_string(),
        }
    }
}

impl traits::MakingUrlFormat for DataFrequency {
    fn generate_url_format(&self) -> String {
        format!("frequency={}", self.to_string())
    }
}


/// contains the elements of frequency formulas.
///
/// This struct is used as an argument for 
/// [`get_advanced_data`](crate::evds_currency::CurrencySeries::get_advanced_data) function.
pub struct AdvancedProcesses {
    pub aggregation_type: AggregationType,
    pub formula: Formula,
    pub data_frequency: DataFrequency,
}

impl AdvancedProcesses {
    /// creates frequency formulas structure named AdvancedProcess.
    /// 
    /// # Example
    /// ```
    ///     use tcmb_evds_c::evds_currency::{AggregationType, Formula, DataFrequency, AdvancedProcesses}
    ///
    ///     let aggregation_type = Aggregation::Average;
    ///     let formula = Formula::Level;
    ///     let data_frequency = DataFrequency::Monthly;
    ///
    ///     let advanced_process = AdvancedProcess::from(aggregation_type, formula, data_frequency);
    /// ```
    pub fn from(
        aggregation_type: AggregationType, 
        formula: Formula, 
        data_frequency: DataFrequency
    ) -> AdvancedProcesses {
        AdvancedProcesses {
            aggregation_type,
            formula,
            data_frequency
        }
    }

    pub(crate) fn get_aggregation_type_as_url_format(&self) -> String {
        self.aggregation_type.generate_url_format()
    }

    pub(crate) fn get_formula_as_url_format(&self) -> String {
        self.formula.generate_url_format()
    }

    pub(crate) fn get_data_frequency_as_url_format(&self) -> String {
        self.data_frequency.generate_url_format()
    }
}
