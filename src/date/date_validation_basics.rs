/// checks day, month and year values are whether valid or not.
pub(crate) fn is_each_value_valid(date: &str) -> bool {

    let max_day_number = 31;
    let max_month_number = 12;
    let min_year_number = 1000;
    let max_year_number = 9999;


    let string_parts = date.split('-');

    if string_parts.count() != 3 { return false; }

    let string_parts = date.split('-');

    let mut count_numbers = 0;

    for part in string_parts {
        let parsed_value = part.parse::<u16>();

        if part.parse::<u16>().is_err() { return false; }
        
        count_numbers += 1;

        let value = parsed_value.unwrap();

        if count_numbers == 1 && value > max_day_number { return false; }
        if count_numbers == 2 && value > max_month_number { return false; }
        if count_numbers == 3 && (value < min_year_number || value > max_year_number) { return false; }
    }

    
    true
}

/// checks format of the given date string is whether valid or not.
pub(crate) fn is_alignment_valid(date: &str) -> bool {

    let dash_initial_index = 2;
    let dash_final_index = 5;
    let min_line_number = 2;
    let max_line_number = 8;


    let mut count_lines = 0;
    let mut count_numbers = 0;

    let char_indices = date.char_indices();

    for (index, character) in char_indices {
        if character == '-' { 
            count_lines += 1; 

            if index != dash_initial_index && index != dash_final_index { return false; }
        }
        if character >= '0' && character <= '9' { count_numbers += 1; }
    }

    if count_lines != min_line_number || count_numbers != max_line_number { return false; }


    true
}
