use super::enum_specific::EnumSpecific;

pub(crate) trait ConvertingToRustEnum<T: EnumSpecific> {
    /// converts implemented type to an enum type declared with T.
    fn convert(&self) -> T;
}
