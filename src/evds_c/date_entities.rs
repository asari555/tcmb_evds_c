use crate::evds_c::common_entities::*;
use crate::evds_c::error_handling::ReturnErrorC;
use crate::traits::checking_string_character::CheckingStringCharacter;


pub(crate) fn check_date_string_validity(
    valid_amount_number: bool, 
    valid_amount_dash: bool, 
    valid_amount_comma: bool, 
) -> Result<(), TcmbEvdsResult> {

    if !valid_amount_number {
        return Err(
            TcmbEvdsResult::generate_result(
                "Error: There is not enough amount of required number in the given date data.".to_string(),
                ReturnErrorC::MissingNumberInDateData,
            )
        );
    }
    if !valid_amount_dash {
        return Err(
            TcmbEvdsResult::generate_result(
                "Error: There are some problems with dashes in the given date data.".to_string(),
                ReturnErrorC::MissingDashInDateData,
            )
        );
    }
    if !valid_amount_comma {
        return Err(
            TcmbEvdsResult::generate_result(
                "Error: There is no comma in the given date data.".to_string(),
                ReturnErrorC::MissingCommaInDateData,
            )
        );
    }
    
    Ok(())
}


pub(crate) fn check_date_data_string_length(date_string: &str) -> Result<DateFormatType, TcmbEvdsResult> {
    
    let single_date_length = 10;
    let multiple_date_max_length = 22;
    let multiple_date_min_length = 21;

    let length = date_string.len();


    if length > multiple_date_max_length { 
        return Err(
            TcmbEvdsResult::generate_result(
                "Error: Length of the given date data is more than expected.".to_string(),
                ReturnErrorC::DateDataExceedingLengthLimit,
            )
        );
    };
    if length > single_date_length && length < multiple_date_min_length {
        return Err(
            TcmbEvdsResult::generate_result(
                "Error: Undefined date data format.
                \nHelp: Please prefer one of the date data format types single and multiple.".to_string(),
                ReturnErrorC::UndefinedDateDataFormat,
            )
        );
    }

    if length == single_date_length { return Ok(DateFormatType::Single); }
    
    Ok(DateFormatType::Multiple)
}



impl CheckingStringCharacter for char {
    fn is_dash(&self) -> bool {
        if *self != '-' { return false; }
    
        true
    }
    fn is_comma(&self) -> bool {
        if *self != ',' { return false; }
    
        true
    }
    fn is_space(&self) -> bool {
        if *self != ' ' { return false; }
    
        true
    }
}



pub(crate) enum DateFormatType {
    Single,
    Multiple,
}

/// checks compatibility of the given date data formats which are single and multiple dates.
///
/// # The required formats
///
/// > **Single:**
/// >> "dd-mm-yyyy" -> "13-12-2011"
///
/// > **Multiple:**
/// >> One comma and/or an empty space. <br />
/// >> *"dd-mm-yyyy,dd-mm-yyyy"* -> *"13-12-2011,13-12-2021"* <br />
/// >> *"dd-mm-yyyy, dd-mm-yyyy"* -> *"13-12-2011, 13-12-2021"*
pub(crate) fn check_date_format(date_string: &str) -> Result<DateFormatType, TcmbEvdsResult> {

    // checking part of the correctness of the both date format types.
    let date_format_type = check_date_data_string_length(date_string)?;

    let count_numeric = date_string.chars().filter(|character| character.is_numeric()).count();
    let count_dash = date_string.chars().filter(|character| character.is_dash()).count();

    // evaluation part of the single date format type.
    if let DateFormatType::Single = date_format_type {
        let valid_amount_number = count_numeric == 8;
        let valid_amount_dash = count_dash == 2;

        // supplies required the parameter value.
        //
        // The gap is the unfilled parameter.
        let gap_filler = true;

        check_date_string_validity(
            valid_amount_number, 
            valid_amount_dash, 
            gap_filler,
        )?;

        return Ok(date_format_type);
    }
    
    // checking and evaluation part of the multiple date format type. 
    let count_comma = date_string.chars().filter(|character| character.is_comma()).count();

    let valid_amount_number = count_numeric == 16 || count_numeric == 8; 
    let valid_amount_comma = count_comma == 1;
    let valid_amount_dash = count_dash == 4;

    check_date_string_validity(valid_amount_number, valid_amount_dash, valid_amount_comma)?;

    Ok(date_format_type)
}

/// divides dates data into two separated date data.
pub(crate) fn parse_dates(dates: &str) -> (&str, &str) {

    let date_format_length = 10;

    let mut split_dates = dates.split(&[','][..]).into_iter();

    let first_date = split_dates.next().unwrap();

    let mut second_date = split_dates.next().unwrap();

    // ignores if there is an empty space between two dates.
    if second_date.len() > date_format_length { second_date = &second_date[1..]; }

    (first_date, second_date)
}
