use crate::evds_currency::frequency_formulas::{AggregationType, DataFrequency, Formula};
use crate::traits::{converting_to_rust_enum::*, enum_specific::*};


/// supplies an aggregation type option to [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data).
#[repr(C)]
pub enum TcmbEvdsAggregationType {
    Average,
    Minimum,
    Maximum,
    Beginning,
    End,
    Cumulative,
}

/// supplies a formula option to [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data).
#[repr(C)]
pub enum TcmbEvdsFormula {
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

/// supplies a data frequency option to [`tcmb_evds_c_get_advanced_data`](crate::tcmb_evds_c_get_advanced_data).
#[repr(C)]
pub enum TcmbEvdsDataFrequency {
    Daily, 
    Business, 
    WeeklyFriday,
    TwiceMonthly, 
    Monthly, 
    Quarterly,
    SemiAnnual, 
    Annual,
}

impl ConvertingToRustEnum<DataFrequency> for TcmbEvdsDataFrequency {
    /// returns `Daily` option by default.
    fn convert(&self) -> DataFrequency {
        match self {
            TcmbEvdsDataFrequency::Business => return DataFrequency::Business,
            TcmbEvdsDataFrequency::WeeklyFriday => return DataFrequency::WeeklyFriday,
            TcmbEvdsDataFrequency::TwiceMonthly => return DataFrequency::TwiceMonthly,
            TcmbEvdsDataFrequency::Monthly => return DataFrequency::Monthly,
            TcmbEvdsDataFrequency::Quarterly => return DataFrequency::Quarterly,
            TcmbEvdsDataFrequency::SemiAnnual => return DataFrequency::SemiAnnual,
            TcmbEvdsDataFrequency::Annual => return DataFrequency::Annual,
            _ => return DataFrequency::Daily,
        }
    }
}

impl ConvertingToRustEnum<Formula> for TcmbEvdsFormula {
    /// returns `Level` option by default.
    fn convert(&self) -> Formula {
        match self {
            TcmbEvdsFormula::PercentageChange => return Formula::PercentageChange,
            TcmbEvdsFormula::Difference => return Formula::Difference,
            TcmbEvdsFormula::YearToYearPercentChange => return Formula::YearToYearPercentChange,
            TcmbEvdsFormula::YearToYearDifferences => return Formula::YearToYearDifferences,
            TcmbEvdsFormula::PercentageChangeByEndOfPreviousYear => return Formula::PercentageChangeByEndOfPreviousYear,
            TcmbEvdsFormula::DifferenceByEndOfPreviousYear => return Formula::DifferenceByEndOfPreviousYear,
            TcmbEvdsFormula::MovingAverage => return Formula::MovingAverage,
            TcmbEvdsFormula::MovingSum => return Formula::MovingSum,
            _ => return Formula::Level,
        }
    }
}

impl ConvertingToRustEnum<AggregationType> for TcmbEvdsAggregationType {
    /// returns `Average` option by default.
    fn convert(&self) -> AggregationType {
        match self {
            TcmbEvdsAggregationType::Minimum => return AggregationType::Minimum,
            TcmbEvdsAggregationType::Maximum => return AggregationType::Maximum,
            TcmbEvdsAggregationType::Beginning => return AggregationType::Beginning,
            TcmbEvdsAggregationType::End => return AggregationType::End,
            TcmbEvdsAggregationType::Cumulative => return AggregationType::Cumulative,
            _ => return AggregationType::Average,
        }
    }
}

impl EnumSpecific for DataFrequency {}
impl EnumSpecific for Formula {}
impl EnumSpecific for AggregationType {}
