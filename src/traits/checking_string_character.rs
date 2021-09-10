pub(crate) trait CheckingStringCharacter {
    fn is_dash(&self) -> bool;
    fn is_comma(&self) -> bool;
    fn is_space(&self) -> bool;
}
