pub(crate) trait MakingString {
    /// stringifies given data structure whether struct or enum.
    fn stringify(&self) -> String;
}
