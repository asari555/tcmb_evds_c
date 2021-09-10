pub(crate) trait MakingList {
    /// generates a list from elements of any structure used.
    fn make_required_list(&self) -> Vec<&str>;
}
