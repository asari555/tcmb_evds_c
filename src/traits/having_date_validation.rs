pub(crate) trait HavingDateValidation {
    /// checks emptiness of given date string/s. 
    fn is_given_date_empty(&self) -> bool;

    /// checks validity of given date string/s.
    fn is_string_length_valid(&self) -> bool;

    // checks overall criteria and includes all the required checking functions.
    fn is_given_date_valid(&self) -> bool;    
}
